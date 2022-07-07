use crate::description::TypeID;
use crate::syntax::{SOAP_NS_ENCODING, SOAP_NS_ENVELOPE};
/**

```http
POST path of control URL HTTP/1.1
HOST: host of control URL:port of control URL
CONTENT-LENGTH: bytes in body
CONTENT-TYPE: text/xml; charset="utf-8"
SOAPACTION: "urn:schemas-upnp-org:service:serviceType:v#actionName"

<?xml version="1.0"?>
<s:Envelope
   xmlns:s="http://schemas.xmlsoap.org/soap/envelope/"
   s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
   <s:Body>
      <u:actionName xmlns:u="urn:schemas-upnp-org:service:serviceType:v">
         <argumentName>in arg value</argumentName>
         other in args and their values go here, if any
      </u:actionName>
   </s:Body>
</s:Envelope>
```
*/
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Action {
    service: TypeID,
    action: String,
}

#[derive(Clone, Debug)]
pub enum Body {
    Action {
        action: Action,
        argumments: HashMap<String, String>,
    },
    Response {
        action: Action,
        argumments: HashMap<String, String>,
    },
    Fault {
        code: String,
        string: String,
        upnp_code: String,
        upnp_description: String,
    },
}

#[derive(Clone, Debug)]
pub struct Envelope {
    #[allow(dead_code)]
    schema: String,
    #[allow(dead_code)]
    encoding_style: String,
    #[allow(dead_code)]
    body: Body,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Action {
    pub fn new(service: TypeID, action: String) -> Self {
        Action { service, action }
    }

    pub fn copy_to(&self, action: String) -> Self {
        Action {
            service: self.service.clone(),
            action,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}#{}", self.service, self.action)
    }
}

// ------------------------------------------------------------------------------------------------

impl Envelope {
    pub fn new(action: Action) -> Self {
        Self::new_with(action, Default::default())
    }

    pub fn new_with(action: Action, argumments: HashMap<String, String>) -> Self {
        Envelope {
            schema: SOAP_NS_ENVELOPE.to_string(),
            encoding_style: SOAP_NS_ENCODING.to_string(),
            body: Body::Action { action, argumments },
        }
    }

    pub fn new_response(action: Action, argumments: HashMap<String, String>) -> Self {
        Envelope {
            schema: SOAP_NS_ENVELOPE.to_string(),
            encoding_style: SOAP_NS_ENCODING.to_string(),
            body: Body::Response { action, argumments },
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
