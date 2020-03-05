use self::super::error::*;
use self::super::name::*;
use self::super::syntax::*;
use self::super::traits::*;
use crate::common::xml::dom_core::rc_cell::{RcRefCell, WeakRefCell};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Node for RefNode {
    fn name(&self) -> Name {
        let ref_self = self.borrow();
        ref_self.i_name.clone()
    }

    fn node_value(&self) -> Option<String> {
        let ref_self = self.borrow();
        ref_self.i_value.clone()
    }

    fn set_node_value(&mut self, value: &str) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = Some(value.to_string());
        Ok(())
    }

    fn unset_node_value(&mut self) -> Result<()> {
        let mut mut_self = self.borrow_mut();
        mut_self.i_value = None;
        Ok(())
    }

    fn node_type(&self) -> NodeType {
        let ref_self = self.borrow();
        ref_self.i_node_type.clone()
    }

    fn parent_node(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_parent_node {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn child_nodes(&self) -> Vec<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_child_nodes.clone()
    }

    fn first_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match ref_self.i_child_nodes.first() {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }

    fn last_child(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match ref_self.i_child_nodes.first() {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }

    fn attributes(&self) -> HashMap<Name, RefNode, RandomState> {
        let ref_self = self.borrow();
        ref_self.i_attributes.clone()
    }

    fn owner_document(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        match &ref_self.i_owner_document {
            None => None,
            Some(node) => node.clone().upgrade(),
        }
    }

    fn insert_before(&mut self, _new_child: RefNode, _ref_child: &RefNode) -> Result<RefNode> {
        unimplemented!()
    }

    fn replace_child(&mut self, _new_child: RefNode, _old_child: &RefNode) -> Result<RefNode> {
        unimplemented!()
    }

    fn append_child(&mut self, new_child: RefNode) -> Result<RefNode> {
        // update child with references
        {
            let mut mut_child = new_child.borrow_mut();
            mut_child.i_parent_node = Some(self.to_owned().downgrade());

            let ref_self = self.borrow();
            mut_child.i_document_element = ref_self.i_document_element.clone();
        }
        let mut mut_self = self.borrow_mut();

        let child_node_type = new_child.node_type();
        if mut_self.i_node_type == NodeType::Document && child_node_type == NodeType::Element {
            // a document may only have one child element
            mut_self.i_document_element = Some(new_child.clone());
        } else {
            mut_self.i_child_nodes.push(new_child.clone());
        }

        let _id_map = mut_self.i_document_element.as_ref().unwrap().borrow_mut();

        // if child_node_type == NodeType::Element {
        //     let child_element = &new_child as &dyn Element;
        //     match child_element.get_attribute(XML_DEFAULT_ID_ATTRIBUTE) {
        //         _ => {}
        //         // TODO: if child is an element and has an ID then add to id_map
        //         Some(_attribute) => (),
        //     }
        // }

        Ok(new_child)
    }

    fn has_child_nodes(&self) -> bool {
        !self.child_nodes().is_empty()
    }

    fn clone_node(&self, _deep: bool) -> Option<RefNode> {
        unimplemented!()
    }

    fn normalize(&mut self) {
        unimplemented!()
    }

    fn is_supported(&self, _feature: String, _version: String) -> bool {
        unimplemented!()
    }

    fn has_attributes(&self) -> bool {
        !self.attributes().is_empty()
    }
}

// ------------------------------------------------------------------------------------------------

impl Attribute for RefNode {}

// ------------------------------------------------------------------------------------------------

impl CharacterData for RefNode {
    fn substring(&self, offset: usize, count: usize) -> Result<String> {
        let ref_self = self.borrow();
        match &ref_self.i_value {
            None => Err(Error::IndexSize),
            Some(data) => Ok(data[offset..offset + count].to_string()),
        }
    }

    fn append(&mut self, new_data: &str) -> Result<()> {
        if !new_data.is_empty() {
            let mut mut_self = self.borrow_mut();
            match &mut_self.i_value {
                None => mut_self.i_value = Some(new_data.to_string()),
                Some(old_data) => mut_self.i_value = Some(format!("{}{}", old_data, new_data)),
            }
        }
        Ok(())
    }

    fn insert(&mut self, offset: usize, new_data: &str) -> Result<()> {
        if !new_data.is_empty() {
            let mut mut_self = self.borrow_mut();
            match &mut_self.i_value {
                None => {
                    if offset != 0 {
                        Err(Error::IndexSize)
                    } else {
                        mut_self.i_value = Some(new_data.to_string());
                        Ok(())
                    }
                }
                Some(old_data) => {
                    if offset >= old_data.len() {
                        Err(Error::IndexSize)
                    } else {
                        mut_self.i_value = Some(format!("{}{}", old_data, new_data));
                        Ok(())
                    }
                }
            }
        } else {
            Ok(())
        }
    }

    fn delete(&mut self, offset: usize, count: usize) -> Result<()> {
        self.replace(offset, count, "")
    }

    fn replace(&mut self, offset: usize, count: usize, replace_data: &str) -> Result<()> {
        if count > 0 {
            let mut mut_self = self.borrow_mut();
            match &mut_self.i_value {
                None => {
                    if offset != 0 {
                        Err(Error::IndexSize)
                    } else {
                        Ok(())
                    }
                }
                Some(old_data) => {
                    if offset >= old_data.len() {
                        Err(Error::IndexSize)
                    } else {
                        let mut new_data = old_data.clone();
                        new_data.replace_range(offset..offset + count, replace_data);
                        mut_self.i_value = Some(new_data);
                        Ok(())
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Comment for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Document for RefNode {
    fn create_attribute(&self, name: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_with(&self, name: &str, value: &str) -> Result<RefNode> {
        let name = Name::from_str(name)?;
        let node_impl = NodeImpl::new_attribute(name, Some(value));
        Ok(RefNode::new(node_impl))
    }

    fn create_attribute_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_attribute(name, None);
        Ok(RefNode::new(node_impl))
    }

    fn create_cdata_section(&self, data: &str) -> Result<RefNode> {
        let node_impl = NodeImpl::new_cdata(data);
        Ok(RefNode::new(node_impl))
    }

    fn create_comment(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_comment(data);
        RefNode::new(node_impl)
    }

    fn create_element(&self, tag_name: &str) -> Result<RefNode> {
        let name = Name::from_str(tag_name)?;
        let node_impl = NodeImpl::new_element(name);
        Ok(RefNode::new(node_impl))
    }

    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_element(name);
        Ok(RefNode::new(node_impl))
    }

    fn create_processing_instruction(&self, target: &str, data: Option<&str>) -> Result<RefNode> {
        let target = Name::from_str(target)?;
        let node_impl = NodeImpl::new_processing_instruction(target, data);
        Ok(RefNode::new(node_impl))
    }

    fn create_text_node(&self, data: &str) -> RefNode {
        let node_impl = NodeImpl::new_comment(data);
        RefNode::new(node_impl)
    }

    fn get_element_by_id(&self, id: &str) -> Option<RefNode> {
        let ref_self = self.borrow();
        let document = ref_self.i_document_element.as_ref().unwrap();
        let id_map = &document.borrow().i_attributes;
        match Name::from_str(id) {
            Ok(id_name) => id_map.get(&id_name).map(|n| n.clone()),
            Err(_) => None,
        }
    }

    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn get_elements_by_tag_name_ns(&self, _namespace_uri: &str, _local_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn doc_type(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_document_type.clone()
    }

    fn document_element(&self) -> Option<RefNode> {
        let ref_self = self.borrow();
        ref_self.i_document_element.clone()
    }

    fn implementation(&self) -> &Implementation {
        &Implementation {}
    }
}

// ------------------------------------------------------------------------------------------------

impl DocumentType for RefNode {
    fn public_id(&self) -> Option<String> {
        let as_element = self as &dyn Element;
        as_element.get_attribute(XML_DOCTYPE_PUBLIC)
    }

    fn system_id(&self) -> Option<String> {
        let as_element = self as &dyn Element;
        as_element.get_attribute(XML_DOCTYPE_SYSTEM)
    }
}

// ------------------------------------------------------------------------------------------------

impl Element for RefNode {
    fn get_attribute(&self, name: &str) -> Option<String> {
        match Name::from_str(name) {
            Ok(attr_name) => {
                let self_copy = self.clone();
                let self_copy = self_copy.borrow();
                match self_copy.i_attributes.get(&attr_name) {
                    None => None,
                    Some(attr_node) => {
                        let attribute = attr_node.borrow();
                        match &attribute.i_value {
                            None => None,
                            Some(value) => Some(value.clone()),
                        }
                    }
                }
            }
            Err(_) => None,
        }
    }

    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()> {
        let attr_name = Name::from_str(name)?;
        let attr_node = NodeImpl::new_attribute(attr_name, Some(value));
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute(&mut self, name: &str) -> Result<()> {
        let _attr_name = Name::from_str(name)?;
        unimplemented!()
    }

    fn get_attribute_node(&self, _name: &str) -> Option<RefNode> {
        unimplemented!()
    }

    fn set_attribute_node(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        let mut mut_self = self.borrow_mut();
        mut_self
            .i_attributes
            .insert(new_attribute.name(), new_attribute.clone());
        Ok(new_attribute)
    }

    fn remove_attribute_node(&mut self, _old_attribute: RefNode) -> Result<RefNode> {
        unimplemented!()
    }

    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String> {
        unimplemented!()
    }

    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()> {
        let attr_name = Name::new_ns(namespace_uri, qualified_name)?;
        let attr_node = NodeImpl::new_attribute(attr_name, Some(value));
        self.set_attribute_node(RefNode::new(attr_node)).map(|_| ())
    }

    fn remove_attribute_ns(&mut self, _namespace_uri: &str, _local_name: &str) -> Result<()> {
        unimplemented!()
    }

    fn get_attribute_node_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<RefNode> {
        unimplemented!()
    }

    fn set_attribute_node_ns(&mut self, new_attribute: RefNode) -> Result<RefNode> {
        self.set_attribute_node(new_attribute)
    }

    fn get_elements_by_tag_name_ns(&self, _namespace_uri: &str, _local_name: &str) -> Vec<RefNode> {
        unimplemented!()
    }

    fn has_attribute(&self, _name: &str) -> bool {
        unimplemented!()
    }

    fn has_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> bool {
        unimplemented!()
    }
}

// ------------------------------------------------------------------------------------------------

impl ProcessingInstruction for RefNode {}

// ------------------------------------------------------------------------------------------------

impl Display for RefNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.node_type() {
            NodeType::Element => {
                let element = self as &dyn Element;
                write!(f, "{}{}", XML_ELEMENT_START_START, element.name())?;
                for attr in element.attributes().values() {
                    write!(f, " {}", attr.to_string())?;
                }
                write!(f, "{}", XML_ELEMENT_START_END)?;
                for child in element.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                write!(
                    f,
                    "{}{}{}",
                    XML_ELEMENT_END_START,
                    element.name(),
                    XML_ELEMENT_END_END
                )
            }
            NodeType::Attribute => {
                let attribute = self as &dyn Attribute;
                write!(
                    f,
                    "\"{}\"=\"{}\"",
                    attribute.name(),
                    attribute.value().unwrap()
                )
            }
            NodeType::Text => {
                let char_data = self as &dyn CharacterData;
                match char_data.data() {
                    None => write!(f, ""),
                    Some(data) => write!(f, "{}", data),
                }
            }
            NodeType::CData => {
                let char_data = self as &dyn CharacterData;
                match char_data.data() {
                    None => write!(f, ""),
                    Some(data) => write!(f, "{} {} {}", XML_COMMENT_START, data, XML_COMMENT_END),
                }
            }
            NodeType::ProcessingInstruction => {
                let pi = self as &dyn ProcessingInstruction;
                match pi.data() {
                    None => write!(f, "{}{}{}>", XML_PI_START, self.target(), XML_PI_END),
                    Some(data) => {
                        write!(f, "{}{} {}{}>", XML_PI_START, pi.target(), data, XML_PI_END)
                    }
                }
            }
            NodeType::Comment => {
                let char_data = self as &dyn CharacterData;
                match char_data.data() {
                    None => write!(f, ""),
                    Some(data) => write!(f, "{}{}{}", XML_CDATA_START, data, XML_CDATA_END),
                }
            }
            NodeType::Document => {
                for child in self.child_nodes() {
                    write!(f, "{}", child.to_string())?;
                }
                let document = self as &dyn Document;
                match document.document_element() {
                    None => write!(f, ""),
                    Some(document_element) => write!(f, "{}", document_element),
                }
            }
            NodeType::DocumentType => {
                let doc_type = self as &dyn DocumentType;
                write!(
                    f,
                    "{} {} {} {} {}",
                    XML_DOCTYPE_START,
                    doc_type.name(),
                    match doc_type.public_id() {
                        None => "".to_string(),
                        Some(public_id) => format!("{} {}", XML_DOCTYPE_PUBLIC, public_id),
                    },
                    match doc_type.system_id() {
                        None => "".to_string(),
                        Some(system_id) => format!("{} {}", XML_DOCTYPE_SYSTEM, system_id),
                    },
                    XML_DOCTYPE_END
                )
            }
            _ => write!(f, ""),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl NodeImpl {
    pub fn new_element(name: Name) -> Self {
        Self {
            i_node_type: NodeType::Element,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub fn new_attribute(name: Name, value: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::Attribute,
            i_name: name,
            i_value: value.map(|v| v.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub fn new_text(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Text,
            i_name: Name::for_text(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub fn new_cdata(data: &str) -> Self {
        Self {
            i_node_type: NodeType::CData,
            i_name: Name::for_cdata(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub fn new_processing_instruction(target: Name, data: Option<&str>) -> Self {
        Self {
            i_node_type: NodeType::ProcessingInstruction,
            i_name: target,
            i_value: data.map(|v| v.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub fn new_comment(data: &str) -> Self {
        Self {
            i_node_type: NodeType::Comment,
            i_name: Name::for_cdata(),
            i_value: Some(data.to_string()),
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        }
    }
    pub fn new_document(name: Name, doc_type: Option<RefNode>) -> Self {
        Self {
            i_node_type: NodeType::Document,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: doc_type,
        }
    }
    pub fn new_document_type(name: Name, public_id: &str, system_id: &str) -> Self {
        let new_doc_type = Self {
            i_node_type: NodeType::DocumentType,
            i_name: name,
            i_value: None,
            i_parent_node: None,
            i_owner_document: None,
            i_attributes: Default::default(),
            i_child_nodes: vec![],
            i_document_element: None,
            i_document_type: None,
        };
        let mut ref_node: RefNode = RcRefCell::new(new_doc_type);
        let as_element = &mut ref_node as &mut dyn Element;
        as_element.set_attribute(XML_DOCTYPE_PUBLIC, public_id);
        as_element.set_attribute(XML_DOCTYPE_SYSTEM, system_id);
        ref_node.unwrap()
    }
}

// ------------------------------------------------------------------------------------------------

impl Implementation {
    ///
    /// Create a new document node; this is taken from the `DOMImplementation` interface.
    ///
    /// # Specification
    ///
    /// Creates an XML Document object of the specified type with its document element. HTML-only DOM
    /// implementations do not need to implement this method. **introduced in DOM Level 2**
    ///
    /// ## Parameters
    ///
    /// * `namespaceURI` of type `DOMString`: The namespace URI of the document element to create.
    /// * `qualifiedName` of type `DOMString`: The qualified name of the document element to be created.
    /// * `doctype` of type `DocumentType`: The type of document to be created or null.
    ///   When doctype is not null, its Node.ownerDocument attribute is set to the document being created.
    ///
    /// ## Return Value
    ///
    /// `Document`: A new Document object.
    ///
    /// ## Exceptions
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal character.
    /// * `NAMESPACE_ERR`: Raised if the qualifiedName is malformed, if the qualifiedName has a prefix
    ///   and the namespaceURI is null, or if the qualifiedName has a prefix that is "xml" and the
    ///   namespaceURI is different from "http://www.w3.org/XML/1998/namespace".
    /// * `WRONG_DOCUMENT_ERR`: Raised if doctype has already been used with a different document or
    ///   was created from a different implementation.
    ///
    /// ## Mapping
    ///
    /// In this function the `doctype` parameter takes a tuple of strings representing the public and
    /// system IDs of the document type.
    ///
    pub fn create_document(
        namespace_uri: &str,
        qualified_name: &str,
        doc_type: Option<RefNode>,
    ) -> Result<RefNode> {
        let name = Name::new_ns(namespace_uri, qualified_name)?;
        let node_impl = NodeImpl::new_document(name, doc_type);
        Ok(RefNode::new(node_impl))
    }

    ///
    /// Create a new document type node; this is taken from the `DOMImplementation` interface.
    ///
    /// # Specification
    ///
    /// Creates an empty DocumentType node. Entity declarations and notations are not made available.
    /// Entity reference expansions and default attribute additions do not occur. It is expected that a
    /// future version of the DOM will provide a way for populating a DocumentType. **introduced in DOM
    /// Level 2**
    ///
    /// HTML-only DOM implementations do not need to implement this method.
    ///
    /// ## Parameters
    ///
    /// * `qualifiedName` of type `DOMString`: The qualified name of the document type to be created.
    /// * `publicId` of type `DOMString`: The external subset public identifier.
    /// * `systemId` of type `DOMString`: The external subset system identifier.
    ///
    /// ## Return Value
    ///
    /// `DocumentType` -- A new DocumentType node with Node.ownerDocument set to null.
    ///
    /// ## Exceptions
    ///
    /// * `INVALID_CHARACTER_ERR`: Raised if the specified qualified name contains an illegal character.
    /// * `NAMESPACE_ERR`: Raised if the qualifiedName is malformed.
    ///
    pub fn create_document_type(
        qualified_name: &str,
        public_id: &str,
        system_id: &str,
    ) -> Result<RefNode> {
        let name = Name::from_str(qualified_name)?;
        let node_impl = NodeImpl::new_document_type(name, public_id, system_id);
        Ok(RefNode::new(node_impl))
    }

    pub fn has_feature(_feature: String, _version: String) -> bool {
        unimplemented!()
    }
}
