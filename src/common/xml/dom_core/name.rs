use self::super::error::*;
use self::super::syntax::*;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::{from_utf8, FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to attributes `localName`, `namespaceURI`, and `prefix` on the DOM `Node` interface.
///
/// # Specification
///
/// `localName` **of type `DOMString`, readonly, introduced in DOM Level 2**
///
/// > Returns the local part of the qualified name of this node.
/// > For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM
/// > Level 1 method, such as createElement from the Document interface, this is always null.
///
/// `namespaceURI` **of type `DOMString`, readonly, introduced in DOM Level 2**
/// > The namespace URI of this node, or null if it is unspecified.
/// > This is not a computed value that is the result of a namespace lookup based on an examination
/// > of the namespace declarations in scope. It is merely the namespace URI given at creation time.
/// > For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM
/// > Level 1 method, such as createElement from the Document interface, this is always null.
///
/// > **Note:** Per the Namespaces in XML Specification an attribute does not inherit its namespace
/// > from the element it is attached to. If an attribute is not explicitly given a namespace, it
/// > simply has no namespace.
///
/// `prefix` **of type `DOMString`, introduced in DOM Level 2**
///
/// > The namespace prefix of this node, or null if it is unspecified.
/// > Note that setting this attribute, when permitted, changes the nodeName attribute, which holds
/// > the qualified name, as well as the tagName and name attributes of the Element and Attr
/// > interfaces, when applicable.
/// > Note also that changing the prefix of an attribute that is known to have a default value,
/// > does not make a new attribute with the default value and the original prefix appear, since
/// > the namespaceURI and localName do not change.
/// > For nodes of any type other than ELEMENT_NODE and ATTRIBUTE_NODE and nodes created with a DOM
/// > Level 1 method, such as createElement from the Document interface, this is always null.
/// >
/// > **Exceptions on setting**
/// >
/// > * `INVALID_CHARACTER_ERR`: Raised if the specified prefix contains an illegal character.
/// > * `NO_MODIFICATION_ALLOWED_ERR`: Raised if this node is readonly.
/// > * `NAMESPACE_ERR`: Raised if the specified prefix is malformed, if the namespaceURI of this
/// >   node is null, if the specified prefix is "xml" and the namespaceURI of this node is
/// >   different from "http://www.w3.org/XML/1998/namespace", if this node is an attribute and the
/// >   specified prefix is "xmlns" and the namespaceURI of this node is different from
/// >   "http://www.w3.org/2000/xmlns/", or if this node is an attribute and the qualifiedName of
/// >   this node is "xmlns".
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) namespace_uri: Option<String>,
    pub(crate) prefix: Option<String>,
    pub(crate) local_name: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.prefix {
            Some(prefix) => write!(f, "{}{}{}", prefix, XML_NS_SEPARATOR, self.local_name),
            None => write!(f, "{}", self.local_name),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl FromStr for Name {
    type Err = Error;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        let parts = value
            .split(XML_NS_SEPARATOR)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        match parts.len() {
            1 => Ok(Name::new(
                Name::check_part(parts.get(0).unwrap())?,
                None,
                None,
            )),
            2 => Ok(Name::new(
                Name::check_part(parts.get(0).unwrap())?,
                Some(Name::check_part(parts.get(1).unwrap())?),
                None,
            )),
            _ => Err(Error::Syntax),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl TryFrom<&[u8]> for Name {
    type Error = Error;

    fn try_from(value: &[u8]) -> StdResult<Self, Self::Error> {
        match from_utf8(value) {
            Ok(str) => Self::from_str(str),
            Err(e) => {
                error!("Could not convert from UTF-8, error {:?}", e);
                Err(Error::InvalidCharacter)
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Name {
    pub fn new_ns(namespace_uri: &str, qualified_name: &str) -> Result<Self> {
        let mut parsed = Name::from_str(qualified_name)?;
        parsed.namespace_uri = Some(Self::check_namespace_uri(namespace_uri, &parsed.prefix)?);
        Ok(parsed)
    }

    fn new(local_name: String, prefix: Option<String>, namespace_uri: Option<String>) -> Self {
        Self {
            namespace_uri,
            prefix,
            local_name,
        }
    }

    fn check_part(part: &str) -> Result<String> {
        // below ranges are always valid for XML 1.0 documents
        // from https://en.wikipedia.org/wiki/XML#Valid_characters
        if part.chars().all(|c| {
            c == '\u{0009}'
                || c == '\u{000A}'
                || c == '\u{000D}'
                || (c >= '\u{0020}' && c <= '\u{D7FF}')
                || (c >= '\u{10000}' && c <= '\u{10FFF}')
        }) {
            Ok(part.to_string())
        } else {
            Err(Error::InvalidCharacter)
        }
    }

    fn check_namespace_uri(namespace_uri: &str, prefix: &Option<String>) -> Result<String> {
        if namespace_uri.is_empty() {
            Err(Error::Namespace)
        } else if let Some(prefix) = prefix {
            if (prefix == XML_NS_ATTRIBUTE && namespace_uri != XML_NS_URI)
                || (prefix == XMLNS_NS_ATTRIBUTE && namespace_uri != XMLNS_NS_URI)
            {
                Err(Error::Namespace)
            } else {
                Ok(namespace_uri.to_string())
            }
        } else {
            Ok(namespace_uri.to_string())
        }
    }

    pub fn for_cdata() -> Self {
        Self::new(XML_NAME_CDATA.to_string(), None, None)
    }

    pub fn for_comment() -> Self {
        Self::new(XML_NAME_COMMENT.to_string(), None, None)
    }

    pub fn for_document() -> Self {
        Self::new(XML_NAME_DOCUMENT.to_string(), None, None)
    }

    pub fn for_text() -> Self {
        Self::new(XML_NAME_TEXT.to_string(), None, None)
    }

    pub fn namespace_uri(&self) -> &Option<String> {
        &self.namespace_uri
    }

    pub fn local_name(&self) -> &String {
        &self.local_name
    }

    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    pub fn set_prefix(&mut self, new_prefix: &str) -> Result<()> {
        self.prefix = Some(new_prefix.to_string());
        Ok(())
    }

    pub fn unset_prefix(&mut self) -> Result<()> {
        self.prefix = None;
        Ok(())
    }
}
