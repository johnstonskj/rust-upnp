/*!
What's this all about then?
*/

use crate::common::xml::write::*;
use crate::error::{xml_error, Error};
use crate::syntax::{
    XML_ATTR_SEND_EVENTS, XML_ELEM_ACTION, XML_ELEM_ACTION_LIST, XML_ELEM_ALLOWED_LIST,
    XML_ELEM_ALLOWED_RANGE, XML_ELEM_ALLOWED_VALUE, XML_ELEM_ARGUMENT, XML_ELEM_ARGUMENT_LIST,
    XML_ELEM_DATA_TYPE, XML_ELEM_DEFAULT_VALUE, XML_ELEM_DIRECTION, XML_ELEM_MAXIMUM,
    XML_ELEM_MINIMUM, XML_ELEM_NAME, XML_ELEM_REL_STATE_VARIABLE, XML_ELEM_RETVAL, XML_ELEM_SPCD,
    XML_ELEM_STATE_TABLE, XML_ELEM_STATE_VARIABLE, XML_ELEM_STEP, XML_NS_SERVICE,
};
use crate::SpecVersion;
use quick_xml::Writer;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Direction {
    In,
    Out,
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub name: String,
    pub direction: Direction,
    pub return_value: bool,
    pub related_state_variable: String,
}

#[derive(Clone, Debug)]
pub struct Action {
    pub name: String,
    pub argument_list: Vec<Argument>,
}

#[derive(Clone, Debug)]
pub enum AllowedValue {
    List {
        values: Vec<String>,
    },
    Range {
        minimum: String,
        maximum: String,
        step: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub struct StateVariable {
    pub send_events: bool,
    pub name: String,
    pub data_type: String,
    pub default_value: Option<String>,
    pub allowed_values: Option<AllowedValue>,
}

#[derive(Clone, Debug)]
pub struct Spcd {
    pub spec_version: SpecVersion,
    pub action_list: Vec<Action>,
    pub service_state_table: Vec<StateVariable>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn to_writer<T: Write>(root: &Spcd, writer: T) -> Result<T, Error> {
    root.write_root(writer)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T: Write> Writable<T> for Argument {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let argument = start_element(writer, XML_ELEM_ARGUMENT).map_err(xml_error)?;

        text_element(writer, XML_ELEM_NAME, self.name.as_bytes()).map_err(xml_error)?;

        text_element(
            writer,
            XML_ELEM_DIRECTION,
            match &self.direction {
                Direction::In => "in".as_bytes(),
                Direction::Out => "out".as_bytes(),
            },
        )
        .map_err(xml_error)?;

        if self.return_value {
            element(writer, XML_ELEM_RETVAL).map_err(xml_error)?;
        }

        text_element(
            writer,
            XML_ELEM_REL_STATE_VARIABLE,
            self.related_state_variable.as_bytes(),
        )
        .map_err(xml_error)?;

        argument.end(writer).map_err(xml_error)
    }
}

impl<T: Write> Writable<T> for Action {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let action = start_element(writer, XML_ELEM_ACTION).map_err(xml_error)?;

        text_element(writer, XML_ELEM_NAME, self.name.as_bytes()).map_err(xml_error)?;

        if !&self.argument_list.is_empty() {
            let list = start_element(writer, XML_ELEM_ARGUMENT_LIST).map_err(xml_error)?;
            for argument in &self.argument_list {
                argument.write(writer)?;
            }
            list.end(writer).map_err(xml_error)?;
        }

        action.end(writer).map_err(xml_error)
    }
}

impl<T: Write> Writable<T> for AllowedValue {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        match self {
            AllowedValue::List { values } => {
                let list = start_element(writer, XML_ELEM_ALLOWED_LIST).map_err(xml_error)?;
                for value in values {
                    text_element(writer, XML_ELEM_ALLOWED_VALUE, value.as_bytes())
                        .map_err(xml_error)?;
                }
                list.end(writer).map_err(xml_error)
            }
            AllowedValue::Range {
                minimum,
                maximum,
                step,
            } => {
                let range = start_element(writer, XML_ELEM_ALLOWED_RANGE).map_err(xml_error)?;

                text_element(writer, XML_ELEM_MINIMUM, minimum.as_bytes()).map_err(xml_error)?;

                text_element(writer, XML_ELEM_MAXIMUM, maximum.as_bytes()).map_err(xml_error)?;

                if let Some(step) = step {
                    text_element(writer, XML_ELEM_STEP, step.as_bytes()).map_err(xml_error)?;
                }
                range.end(writer).map_err(xml_error)
            }
        }
    }
}

impl<T: Write> Writable<T> for StateVariable {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let variable = start_element_with(
            writer,
            XML_ELEM_STATE_VARIABLE,
            vec![(
                XML_ATTR_SEND_EVENTS,
                if self.send_events { "yes" } else { "no" },
            )],
        ).map_err(xml_error)?;

        text_element(writer, XML_ELEM_NAME, self.name.as_bytes()).map_err(xml_error)?;

        text_element(writer, XML_ELEM_DATA_TYPE, self.data_type.as_bytes()).map_err(xml_error)?;

        if let Some(default_value) = &self.default_value {
            text_element(writer, XML_ELEM_DEFAULT_VALUE, default_value.as_bytes()).map_err(xml_error)?;
        }

        if let Some(allowed) = &self.allowed_values {
            allowed.write(writer)?;
        }

        variable.end(writer).map_err(xml_error)
    }
}

impl<T: Write> RootWritable<T> for Spcd {}

impl<T: Write> Writable<T> for Spcd {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let root = start_ns_element(writer, XML_ELEM_SPCD, XML_NS_SERVICE, None).map_err(xml_error)?;

        self.spec_version.write(writer)?;

        if !&self.action_list.is_empty() {
            let list = start_element(writer, XML_ELEM_ACTION_LIST).map_err(xml_error)?;
            for action in &self.action_list {
                action.write(writer)?;
            }
            list.end(writer).map_err(xml_error)?;
        }

        let list = start_element(writer, XML_ELEM_STATE_TABLE).map_err(xml_error)?;
        for variable in &self.service_state_table {
            variable.write(writer)?;
        }
        list.end(writer).map_err(xml_error)?;

        root.end(writer).map_err(xml_error)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------
