// ------------------------------------------------------------------------------------------------
// Pure Syntactic Tokens
// ------------------------------------------------------------------------------------------------

pub const XML_PI_START: &str = "<?";
pub const XML_PI_END: &str = ">";

pub const XML_COMMENT_START: &str = "<!--";
pub const XML_COMMENT_END: &str = "-->";

pub const XML_CDATA_START: &str = "<![CDATA[";
pub const XML_CDATA_END: &str = "]]>";

pub const XML_DOCTYPE_START: &str = "<!DOCTYPE";
pub const XML_DOCTYPE_END: &str = ">";
//pub const XML_DOCTYPE_ENTITY_START: &str = "[";
//pub const XML_DOCTYPE_ENTITY_END: &str = "]";
pub const XML_DOCTYPE_PUBLIC: &str = "PUBLIC";
pub const XML_DOCTYPE_SYSTEM: &str = "SYSTEM";

pub const XML_ELEMENT_START_START: &str = "<";
pub const XML_ELEMENT_START_END: &str = ">";
pub const XML_ELEMENT_END_START: &str = "</";
pub const XML_ELEMENT_END_END: &str = ">";

//pub const XML_EMPTY: &str = "";

// ------------------------------------------------------------------------------------------------
// ID/Ref Support
// ------------------------------------------------------------------------------------------------

//pub const XML_DEFAULT_ID_ATTRIBUTE: &str = "id";

//pub const XML_DEFAULT_IDREF_ATTRIBUTE: &str = "idref";

// ------------------------------------------------------------------------------------------------
// Namespace Support
// ------------------------------------------------------------------------------------------------

pub const XML_NS_ATTRIBUTE: &str = "xml";
pub const XML_NS_URI: &str = "http://www.w3.org/XML/1998/namespace";

pub const XMLNS_NS_ATTRIBUTE: &str = "xmlns";
pub const XMLNS_NS_URI: &str = "http://www.w3.org/2000/xmlns/";

pub const XML_NS_SEPARATOR: &str = ":";

// ------------------------------------------------------------------------------------------------
// DOM Node Names
// ------------------------------------------------------------------------------------------------

pub const XML_NAME_CDATA: &str = "#cdata-section";
pub const XML_NAME_COMMENT: &str = "#comment";
pub const XML_NAME_DOCUMENT: &str = "#document";
pub const XML_NAME_TEXT: &str = "#text";
