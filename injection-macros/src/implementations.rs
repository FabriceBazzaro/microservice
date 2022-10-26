use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;

fn crate_or_name(name: String) -> syn::Ident {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    if crate_name == name {
        syn::Ident::new(&"crate", proc_macro2::Span::call_site())
    }
    else {
        syn::Ident::new(&name, proc_macro2::Span::call_site())
    }
}

pub fn impl_injectable(attributes: &Vec<syn::Ident>, ast: &mut syn::Item) -> TokenStream {
    match ast {
	syn::Item::Trait(ref mut trait_data) => {
            let trait_name = trait_data.ident.clone();
            let name_crate = crate_or_name("microservice".to_string());
            // Recursive register function (a call is done for each constrained trait in the attribute list)
            trait_data.items.push(syn::TraitItem::Verbatim(quote! {

                // Register the structure whith the current trait in the registry
                fn register_trait<T>(component_ref: std::sync::Arc<std::sync::Mutex<T>>, registry: &mut #name_crate::injection::Registry) where T: #trait_name +  #(#attributes +)* 'static, Self: Sized {
                    #(<Self as #attributes>::register_trait(component_ref.clone(), registry);)*
                    // #name_crate::trace!("Register trait {}", std::stringify!(#trait_name));
                    registry.register_with_type::<dyn #trait_name>(component_ref);
                }

                // Return if the structure implement a trait
                fn is_trait(id: std::any::TypeId) -> bool where Self: Sized + 'static {
                    std::any::TypeId::of::<dyn #trait_name>() == id #(|| <Self as #attributes>::is_trait(id))*
                }
            }));
	    let output = quote! {
		#ast
                impl std::fmt::Debug for dyn #trait_name {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.debug_struct(std::stringify!(#trait_name)).finish()
                    }
                }
	    };
            proc_macro::TokenStream::from(output)
	},
	syn::Item::Struct(ref mut struct_data) => {
            let struct_name = struct_data.ident.clone();

            // Extract optional generic parts
            let generics = struct_data.generics.clone();
            let sgenerics: Vec<TokenStream2> = generics.params.clone().into_iter().map(|g| {
                match g {
                    syn::GenericParam::Type(t) => { let t = t.ident;  quote!{ #t } },
                    syn::GenericParam::Lifetime(l) => { let l = l.lifetime; quote!{ #l } },
                    syn::GenericParam::Const(c) => { let c = c.ident; quote!{ #c } }
                }
            }).collect();
            let where_clause = generics.where_clause.clone();
            let name_crate = crate_or_name("microservice".to_string());

            // Compose the result
	    let output = quote! {
		#ast
                impl #generics #name_crate::injection::Component for #struct_name<#(#sgenerics),*> #where_clause {
                    // Function to register the structure and all its traits in the registry
                    fn register(component_ref: std::sync::Arc<std::sync::Mutex<Self>>, registry: &mut #name_crate::injection::Registry) where Self: Sized + 'static {
                        registry.register_with_type::<#struct_name<#(#sgenerics),*>>(component_ref.clone());
                        #(<Self as #attributes>::register_trait(component_ref.clone(), registry);)*
                        // #name_crate::trace!("Register struct {}", std::stringify!(#struct_name));
                    }

                    // Function to return if a trait is implemented in the structure
                    fn struct_impl_trait<_TRAIT_>() -> bool where _TRAIT_: ?Sized + 'static, Self: Sized + 'static {
                        false #(|| <Self as #attributes>::is_trait(std::any::TypeId::of::<_TRAIT_>()))*
                    }
                }
	    };
            proc_macro::TokenStream::from(output)
	},
	_ => {
	    panic!("This is not a struct nor a trait")
	}
    }
}

fn inject_method(method_data: &syn::ImplItemMethod) -> syn::ImplItem {
    let name_crate = crate_or_name("microservice".to_string());
    let mut new_signature = method_data.sig.clone();
    new_signature.ident = syn::Ident::new(&format!("{}{}", method_data.sig.ident.to_string(), "_from_reg"), proc_macro2::Span::call_site());
    new_signature.inputs = syn::punctuated::Punctuated::new();
    new_signature.inputs.push(syn::parse2(quote! { registry: &mut #name_crate::injection::Registry }).unwrap());
    let inputs = method_data.sig.inputs.clone().into_iter().map(|_| quote! { registry.get()?.clone() }).reduce(|accum, item| { quote!{ #accum, #item} }).unwrap_or(TokenStream2::new());
    let output = quote! {
        #new_signature {
            Self::new(#inputs)
        }
    };
    syn::ImplItem::Method(syn::parse2(output).expect("failed to parse generated function"))
}

pub fn impl_injector(_attributes: &Vec<syn::Ident>, ast: &mut syn::Item) -> TokenStream {
    let name_crate = crate_or_name("microservice".to_string());
    match ast {
        syn::Item::Impl(ref mut impl_data) => {
            let generics = impl_data.generics.clone();
            let ident = impl_data.self_ty.clone();
            let mut injection: Vec<syn::ImplItem> = Vec::new();
            let mut injection_new: Vec<syn::ImplItem> = Vec::new();
            for item in impl_data.items.iter_mut() {
                match item {
                    syn::ImplItem::Method(ref mut method_data) => {
                        let mut found = false;
                        method_data.attrs.retain(|attr| {
                            if quote!(#attr).to_string() == "#[inject]".to_string() {
                                found = true;
                                false
                            }
                            else {
                                true
                            }
                        });
                        if found {
                            if method_data.sig.ident.to_string() == "new".to_string() {
                                injection_new.push(inject_method(&method_data));
                            }
                            else {
                                injection.push(inject_method(&method_data));
                            }
                        }
                    },
                    _ => {}
                }
            };
            impl_data.items.append(&mut injection);
            let output = quote! {
		#ast
                impl #generics #name_crate::injection::Injection for #ident {
                    #( #injection_new )*
                }
            };
            proc_macro::TokenStream::from(output)
        }
        _ => panic!("Injector is only usable on impl blocks")
    }
}
