/*!
One-line description.

More detailed description, with

# Example

*/

use crate::common::xml::udom::{Document, Error as DOMError, Name, Node, NodeType};
use crate::{Error, MessageErrorKind};
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use std::borrow::BorrowMut;
use std::convert::TryFrom;
use std::rc::Rc;
use std::str::from_utf8;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn read_xml(xml: &str) -> Result<Document, Error> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let event_buffer: Vec<u8> = Vec::new();

    read_loop_outer(reader, event_buffer)
}

fn read_loop_outer(
    mut reader: Reader<&[u8]>,
    mut event_buffer: Vec<u8>,
) -> Result<Document, Error> {
    let mut document = Document::default();

    loop {
        match reader.read_event(&mut event_buffer) {
            Ok(Event::Start(ev)) => {
                trace!("read_loop_outer - Event::Start ({:?})", ev);
                document.document_element = Some(try_element_from(ev)?);
                let document = read_loop_inner(reader, event_buffer, document)?;
                return Ok(document);
            }
            Ok(Event::Empty(ev)) => {
                document.document_element = Some(try_element_from(ev)?);
                let document = read_loop_inner(reader, event_buffer, document)?;
                return Ok(document);
            }
            Ok(Event::Decl(ev)) => {
                trace!("read_loop_outer - Event::Decl ({:?})", ev);
                let pi_node = Document::create_xml_declaration("", Some(""), Some(true))?;
                document.children.push(Rc::new(pi_node));
            }
            Ok(Event::PI(ev)) => {
                trace!("read_loop_outer - Event::PI");
                ev.
                let pi_node = Document::create_processing_instruction("", "")?;
                document.children.push(Rc::new(pi_node));
            }
            Ok(Event::DocType(ev)) => {
                trace!("read_loop_outer - Event::DocType ({:?})", ev);
            }
            Ok(Event::Eof) => break,
            Ok(ev) => {
                error!("read_loop_outer, unexpected event {:?}", ev);
                return Err(Error::MessageFormat(MessageErrorKind::XmlFormattingError));
            }
            Err(err) => {
                error!(
                    "read_loop_outer error at position {}: {:?}",
                    reader.buffer_position(),
                    err
                );
                return Err(Error::MessageFormat(MessageErrorKind::XmlFormattingError));
            }
        }
    }
    Err(Error::MessageFormat(MessageErrorKind::XmlFormattingError))
}

fn read_loop_inner(
    mut reader: Reader<&[u8]>,
    mut event_buffer: Vec<u8>,
    document: Document,
) -> Result<Document, Error> {
    let mut current_element = document.document_element.unwrap();
    loop {
        let child_node = match reader.read_event(&mut event_buffer) {
            Ok(Event::Start(ev)) => {
                trace!("read_loop_inner - Event::Start ({:?})", ev);
                let mut new_element = try_element_from(ev)?;
                let c = &mut current_element;

                c.borrow_mut().append_child(c, &mut new_element);
                current_element = new_element;
                None
            }
            Ok(Event::End(ev)) => {
                trace!("read_loop_inner - Event::End ({:?})", ev);
                let name = Name::try_from(from_utf8(ev.name())?)?;
                if name != current_element.name {
                    return Err(Error::MessageFormat(MessageErrorKind::XmlFormattingError));
                } else {
                    match current_element.parent_node.clone() {
                        Some(parent) => current_element = parent,
                        None => return Ok(document),
                    }
                }
                None
            }
            Ok(Event::Empty(ev)) => {
                trace!("read_loop_inner - Event::Empty ({:?})", ev);
                Some(try_element_from(ev)?)
            }
            Ok(Event::Text(ev)) => {
                trace!("read_loop_inner - Event::Text ({:?})", ev);
                Some(Rc::new(Document::create_text_node(&string_from_bytes(
                    ev, &reader,
                )?)))
            }
            Ok(Event::Comment(ev)) => {
                trace!("read_loop_inner - Event::Comment ({:?})", ev);
                Some(Rc::new(Document::create_comment(&string_from_bytes(
                    ev, &reader,
                )?)))
            }
            Ok(Event::CData(ev)) => {
                trace!("read_loop_inner - Event::CData ({:?})", ev);
                Some(Rc::new(Document::create_cdata_section(
                    &string_from_bytes(ev, &reader)?,
                )?))
            }
            Ok(Event::Decl(ev)) => {
                trace!("read_loop_inner - Event::Decl ({:?})", ev);
                None
            }
            Ok(Event::PI(ev)) => {
                trace!("read_loop_inner - Event::PI ({:?})", ev);
                None
            }
            Ok(Event::DocType(ev)) => {
                trace!("read_loop_inner - Event::DocType ({:?})", ev);
                None
            }
            Err(err) => {
                error!(
                    "read_loop_inner error at position {}: {:?}",
                    reader.buffer_position(),
                    err
                );
                return Err(Error::MessageFormat(MessageErrorKind::XmlFormattingError));
            }
            Ok(Event::Eof) => break,
        };
        if let Some(mut child_node) = child_node {
            Rc::make_mut(&mut current_element).append_child(&mut current_element, &mut child_node);
        }
    }

    Err(Error::MessageFormat(MessageErrorKind::XmlFormattingError))
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn string_from_bytes(bytes: BytesText<'_>, reader: &Reader<&[u8]>) -> Result<String, Error> {
    bytes
        .unescape_and_decode(reader)
        .map_err(|_| Error::MessageFormat(MessageErrorKind::InvalidEncoding))
}

fn string_from_utf8(value: &[u8]) -> Result<&str, DOMError> {
    match from_utf8(value) {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("Could not convert from UTF-8, error {:?}", e);
            Err(DOMError::InvalidCharacter)
        }
    }
}

fn try_element_from(ev: BytesStart<'_>) -> Result<Node, DOMError> {
    trace!("try_element_from");
    let name = string_from_utf8(ev.name())?;
    let element = Rc::new(Document::create_element(name)?);

    for maybe in ev.attributes() {
        let attribute = maybe.unwrap();
        let mut attribute_node = Document::create_attribute(string_from_utf8(attribute.key)?)?;
        match &attribute_node.rc_node.node {
            NodeType::Attribute {
                mut value,
                mut owner_element,
                ..
            } => {
                value = string_from_utf8(attribute.value.as_ref())?.to_string();
                owner_element = Some(element);
            }
            _ => return Err(DOMError::Syntax),
        }
        element
            .attributes
            .insert(attribute_node.name, Rc::new(attribute_node));
    }
    Ok(element)
}

impl From<DOMError> for Error {
    fn from(e: DOMError) -> Self {
        error!("XML DOM error: {:?}", e);
        Error::MessageFormat(MessageErrorKind::XmlFormattingError)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;

    #[allow(unused_must_use)]
    fn setup_logging() {
        env_logger::Builder::from_default_env()
            .filter_module("upnp_rs::common::xml::read", LevelFilter::Trace)
            .try_init();
    }

    #[test]
    fn test_xml_read_minimal() {
        const TEST_DOC: &str = "<root></root>";
        setup_logging();
        let doc = read_xml(TEST_DOC);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert!(doc.document_element.is_some());
        assert_eq!(doc.document_element.unwrap().name.to_string(), "root");
        println!("{:#?}", doc);
    }

    #[test]
    fn test_xml_read_minimal_with_namespace() {
        const TEST_DOC: &str = "<root xmlns=\"urn:schemas-upnp-org:device-1-0\"></root>";
        setup_logging();
        let doc = read_xml(TEST_DOC);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert!(doc.document_element.is_some());
        assert_eq!(doc.document_element.unwrap().name.to_string(), "root");
        assert_eq!(doc.document_element.unwrap().attributes.len(), 1);
        let ns = doc.document_element.unwrap().attributes.get(0).unwrap();
        assert_eq!(ns.name.to_string(), "xmlns");
        assert_eq!(ns.value, "urn:schemas-upnp-org:device-1-0");
        println!("{:#?}", doc);
    }

    #[test]
    fn test_xml_read_minimal_with_decl() {
        const TEST_DOC: &str =
            "<?xml version=\"1.0\"?><root xmlns=\"urn:schemas-upnp-org:device-1-0\"></root>";
        setup_logging();
        let doc = read_xml(TEST_DOC);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert!(doc.document_element.is_some());
        assert_eq!(doc.document_element.unwrap().name.to_string(), "root");
        assert_eq!(doc.document_element.unwrap().attributes.len(), 1);
        let ns = doc.document_element.unwrap().attributes.get(0).unwrap();
        assert_eq!(ns.name.to_string(), "xmlns");
        assert_eq!(ns.value, "urn:schemas-upnp-org:device-1-0");
        assert_eq!(doc.processing_instructions.len(), 1);
        let pi = doc.processing_instructions.get(0).unwrap();
        assert_eq!(pi.target, "xml");
        // TODO: fix this assert_eq!(pi.attributes.len(), 3);
        println!("{:#?}", doc);
    }
}
