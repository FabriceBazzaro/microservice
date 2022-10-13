#[cfg(test)]
mod url_tests {
    use microservice::share::SimpleUrl;

    #[test]
    fn url() {
        let aurl = SimpleUrl::new(None, "testhost".into(), None);
        let burl = SimpleUrl::new(Some("http://".into()), "testhost".into(), None);
        let curl = SimpleUrl::new(None, "testhost".into(), Some(8888));
        let durl = SimpleUrl::new(Some("http://".into()), "testhost".into(), Some(8888));

        assert_eq!(aurl.to_string(), "testhost");
        assert_eq!(burl.to_string(), "http://testhost");
        assert_eq!(curl.to_string(), "testhost:8888");
        assert_eq!(durl.to_string(), "http://testhost:8888");
    }
}
