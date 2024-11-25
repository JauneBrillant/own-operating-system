use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    fn is_http(&self) -> bool {
        self.url.starts_with("http://")
    }

    fn extract_host(&self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .split(|c| c == '/' || c == '?')
            .collect::<Vec<&str>>();

        let host_with_port = url_parts[0];
        match host_with_port.find(':') {
            Some(index) => host_with_port[..index].to_string(),
            None => host_with_port.to_string(),
        }
    }

    fn extract_port(&self) -> String {
        let url_parts = self
            .url
            .trim_start_matches("http://")
            .split(|c| c == '/' || c == '?')
            .collect::<Vec<&str>>();

        let host_with_port = url_parts[0];
        match host_with_port.find(':') {
            Some(index) => host_with_port[index + 1..].to_string(),
            None => "80".to_string(),
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/')
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        match self.url.find('?') {
            Some(index) => self.url[index + 1..].to_string(),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8080".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_searchquery() {
        let url = "http://example.com?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_path_searchquery() {
        let url = "http://example.com/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_port_searchquery() {
        let url = "http://example.com:8080?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8080/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_port_path_searchquery() {
        let url = "http://example.com:8080/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_url_host_path_path_path_searchpart() {
        let url = "http://example.com/a/b/c.html?a=1&b=2".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "a/b/c.html".to_string(),
            searchpart: "a=1&b=2".to_string(),
        });
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        let actual = Url::new(url).parse();
        assert_eq!(expected, actual);
    }
}
