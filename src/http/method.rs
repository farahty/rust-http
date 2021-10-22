use std::str::FromStr;

pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    PUT,
    POST,
    DELETE,
    PATCH,
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OPTIONS" => Ok(Self::OPTIONS),
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "PUT" => Ok(Self::PUT),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err("invalid method".to_string()),
        }
    }
}
