/*!
One-line description.

More detailed description, with

# Example

*/

use crate::Error;
use quick_xml::events::{attributes::Attribute, BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Element {
    name: &'static [u8],
}

pub trait Writable<T: Write> {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error>;
}

pub trait RootWritable<T: Write>: Writable<T> {
    fn write_root(&self, writer: T) -> Result<T, Error> {
        let mut xml = Writer::new(writer);

        start(&mut xml)?;

        self.write(&mut xml)?;

        Ok(xml.into_inner())
    }
}

pub const X_DECL_VERSION: &[u8] = b"1.0";

pub const X_ATTR_NAMESPACE: &str = "xmlns";

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn start<T: Write>(writer: &mut Writer<T>) -> Result<(), quick_xml::Error> {
    writer
        .write_event(Event::Decl(BytesDecl::new(X_DECL_VERSION, None, None)))
        .map(|_| ())
}

pub fn element<T: Write>(
    writer: &mut Writer<T>,
    name: &'static [u8],
) -> Result<(), quick_xml::Error> {
    writer.write_event(Event::Start(BytesStart::borrowed_name(name)))?;
    writer
        .write_event(Event::End(BytesEnd::borrowed(name)))
        .map(|_| ())
}

pub fn start_element<T: Write>(
    writer: &mut Writer<T>,
    name: &'static [u8],
) -> Result<Element, quick_xml::Error> {
    writer.write_event(Event::Start(BytesStart::borrowed_name(name)))?;
    Ok(Element { name })
}

pub fn start_ns_element<T: Write>(
    writer: &mut Writer<T>,
    name: &'static [u8],
    namespace: &str,
    prefix: Option<&str>,
) -> Result<Element, quick_xml::Error> {
    let xmlns = [
        X_ATTR_NAMESPACE,
        if prefix.is_some() { ":" } else { "" },
        if let Some(p) = prefix { p } else { "" },
    ]
    .concat();

    start_element_with(writer, name, vec![(xmlns.as_str(), namespace)])?;
    Ok(Element { name })
}

pub fn start_element_with<T: Write>(
    writer: &mut Writer<T>,
    name: &'static [u8],
    attrs: Vec<(&str, &str)>,
) -> Result<Element, quick_xml::Error> {
    let mut element = BytesStart::borrowed_name(name);
    for (name, value) in attrs {
        element.push_attribute(Attribute::from((name, value)));
    }
    writer.write_event(Event::Start(element))?;
    Ok(Element { name })
}

pub fn end_element<T: Write>(writer: &mut Writer<T>, name: &[u8]) -> Result<(), quick_xml::Error> {
    writer
        .write_event(Event::End(BytesEnd::borrowed(name)))
        .map(|_| ())
}

pub fn text_element<T: Write>(
    writer: &mut Writer<T>,
    name: &'static [u8],
    content: &[u8],
) -> Result<(), quick_xml::Error> {
    let element = start_element(writer, name)?;
    writer.write_event(Event::Text(BytesText::from_plain(content)))?;
    element.end(writer)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Element {
    pub fn end<T: Write>(&self, writer: &mut Writer<T>) -> Result<(), quick_xml::Error> {
        end_element(writer, self.name)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
