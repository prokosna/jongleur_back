pub enum FlowType {
    AuthorizationCode,
    Implicit,
    Hybrid,
    Undefined,
}

pub struct ResponseType {
    response_type: Vec<String>,
}

impl ResponseType {
    pub fn from_str(response_type: &String) -> Self {
        let response_types: Vec<String> = response_type.split(" ").map(|x| x.to_string()).collect();
        Self::new(&response_types)
    }

    pub fn new(response_type: &Vec<String>) -> Self {
        ResponseType {
            response_type: response_type.clone(),
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.response_type.clone()
    }

    pub fn flow_type(&self) -> FlowType {
        let has_code = self.has_code();
        let has_token = self.has_token();
        let has_id_token = self.has_id_token();

        if has_code && has_token || has_code && has_id_token {
            FlowType::Hybrid
        } else if !has_code && (has_id_token || has_token) {
            FlowType::Implicit
        } else if has_code {
            FlowType::AuthorizationCode
        } else {
            FlowType::Undefined
        }
    }

    pub fn has_code(&self) -> bool {
        self.response_type.iter().any(|x| x == &"code")
    }

    pub fn has_token(&self) -> bool {
        self.response_type.iter().any(|x| x == &"token")
    }

    pub fn has_id_token(&self) -> bool {
        self.response_type.iter().any(|x| x == &"id_token")
    }
}
