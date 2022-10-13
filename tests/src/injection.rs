#[cfg(test)]
mod injection_tests {
    use std::sync::{Arc, Mutex};
    use microservice::error::*;
    use microservice::constant::Constant;
    use microservice::injection::*;
    use microservice::*;

    #[injectable(Component)]
    trait Trait: Component {
        fn get_name(&self) -> String {
            "Trait".into()
        }

        fn get_value(&self) -> u16;
    }

    #[injectable(Component)]
    trait UnknownTrait: Component {
        fn get_name(&self) -> String {
            "Unknown Trait".into()
        }
    }


    #[derive(PartialEq, Debug)]
    #[injectable(Trait)]
    struct TestComponent {
        pub value: u16
    }

    #[injector]
    impl TestComponent {
        #[inject]
        fn new() -> Result<Self> where Self: Sized + 'static {
            Ok(Self {
                value: 15
            })
        }
    }

    impl Trait for TestComponent {
        fn get_name(&self) -> String {
            format!("{} - {}", "TestComponent", self.value)
        }
        fn get_value(&self) -> u16 {
            self.value
        }
    }


    #[derive(PartialEq, Debug)]
    #[injectable(Trait)]
    struct TestComponent2 {
        pub value: u16
    }

    #[injector]
    impl TestComponent2 {
        #[inject]
        fn new() -> Result<Self> where Self: Sized + 'static {
            Ok(Self {
                value: 10
            })
        }
    }

    impl Trait for TestComponent2 {
        fn get_name(&self) -> String {
            format!("{} - {}", "TestComponent", self.value)
        }
        fn get_value(&self) -> u16 {
            self.value
        }
    }

    #[test]
    fn component_registry_get_from_trait() {
        let mut registry = Registry::new();
        registry.register::<TestComponent>().unwrap();

        let t1: Arc<Mutex<TestComponent>> = registry.get::<TestComponent>().unwrap();
        let t2: Arc<Mutex<dyn Trait>> = registry.get::<dyn Trait>().unwrap();

        assert_eq!((*t1.lock().unwrap()).get_value(), 15);
        assert_eq!((*t2.lock().unwrap()).get_value(), 15);

        registry.register::<TestComponent2>().unwrap();
        let t3: Arc<Mutex<TestComponent2>> = registry.get::<TestComponent2>().unwrap();
        let t4: RegistryError = registry.get::<dyn Trait>().unwrap_err().downcast().unwrap();
        let multiple_expected = RegistryError::MultipleComponentsError { name: std::any::type_name::<dyn Trait>() };

        assert_eq!((*t3.lock().unwrap()).get_value(), 10);
        assert_eq!(t4, multiple_expected);
    }

    #[test]
    fn component_registry_get_validity() {
        let mut registry = Registry::new();
        registry.register::<TestComponent>().unwrap();
        registry.register::<TestComponent>().unwrap();

        let t1: RegistryError = registry.get::<TestComponent>().unwrap_err().downcast().unwrap();
        let t2: RegistryError = registry.get::<dyn UnknownTrait>().unwrap_err().downcast().unwrap();

        let multiple_expected = RegistryError::MultipleComponentsError { name: std::any::type_name::<TestComponent>() };
        let none_expected = RegistryError::NoComponentError { name: std::any::type_name::<dyn UnknownTrait>() };

        assert_eq!(t1, multiple_expected);
        assert_eq!(t2, none_expected);
    }

    #[test]
    fn component_registry_mutability() {
        let mut registry = Registry::new();
        registry.register::<TestComponent>().unwrap();

        let t1: Arc<Mutex<TestComponent>> = registry.get::<TestComponent>().unwrap();
        let t2: Arc<Mutex<dyn Trait>> = registry.get::<dyn Trait>().unwrap();

        (*t1.lock().unwrap()).value = 10;

        assert_eq!((*t2.lock().unwrap()).get_value(), 10);
        let t1_value = (*t1.lock().unwrap()).get_value();
        assert_eq!(t1_value, (*t2.lock().unwrap()).get_value());
    }

    #[test]
    fn microservice_register_retrieve_constant() {
        let mut m: Microservice = Microservice::new();
        m.register_instance(Constant::<String, {hash!("CONFIG_FILE")}>::new("./config.yaml".into()));
        let c = m.get::<Constant<String, {hash!("CONFIG_FILE")}>>().unwrap();
        assert_eq!(c.lock().unwrap().value, "./config.yaml");
    }
}
