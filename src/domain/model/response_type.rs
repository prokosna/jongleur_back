use domain::error::domain as ed;

/// `FlowType` is the enum that represents the flow type in the context of OpenID Connect.
pub enum FlowType {
    Undefined,
    AuthorizationCode,
    Implicit,
    Hybrid,
}

/// `ResponseType` is the type contains a list of response_type such as token, id_token, code...
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseType {
    response_type: Vec<String>,
}

impl ResponseType {
    /// Returns new ResponseType created from the value of response_type.
    pub fn from_str(response_type: &String) -> Result<Self, ed::Error> {
        let response_types: Vec<String> = response_type.split(" ").map(|x| x.to_string()).collect();
        let rt = Self::new(&response_types);
        match rt.flow_type() {
            FlowType::Undefined => Err(ed::ErrorKind::InvalidRequest(format!(
                "Invalid response_type: {}",
                response_type
            )).into()),
            _ => Ok(rt),
        }
    }

    /// Returns Vec<String> that contains response_type values.
    pub fn to_vec(&self) -> Vec<String> {
        self.response_type.clone()
    }

    /// Returns new `FlowType` of this `ResponseType`.
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

    /// Returns true if this has `code` in the response_type.
    pub fn has_code(&self) -> bool {
        self.response_type.iter().any(|x| x == &"code")
    }

    /// Returns true if this has `token` in the response_type.
    pub fn has_token(&self) -> bool {
        self.response_type.iter().any(|x| x == &"token")
    }

    /// Returns true if this has `id_token` in the response_type.
    pub fn has_id_token(&self) -> bool {
        self.response_type.iter().any(|x| x == &"id_token")
    }

    pub fn new(response_type: &Vec<String>) -> Self {
        ResponseType {
            response_type: response_type.clone(),
        }
    }
}
