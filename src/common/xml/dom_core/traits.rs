use self::super::error::*;
use self::super::name::*;
use self::super::rc_cell::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

pub trait Attribute: Node {
    fn value(&self) -> Option<String> {
        Node::node_value(self)
    }
    fn set_value(&mut self, value: &str) -> Result<()> {
        Node::set_node_value(self, value)
    }
    fn unset_value(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    fn specified(&self) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------

pub trait CharacterData: Node {
    fn length(&self) -> usize {
        match self.data() {
            None => 0,
            Some(s) => s.len(),
        }
    }
    fn data(&self) -> Option<String> {
        Node::node_value(self)
    }
    fn set_data(&mut self, data: &str) -> Result<()> {
        Node::set_node_value(self, data)
    }
    fn unset_data(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    fn substring(&self, offset: usize, count: usize) -> Result<String>;
    fn append(&mut self, data: &str) -> Result<()>;
    fn insert(&mut self, offset: usize, data: &str) -> Result<()>;
    fn delete(&mut self, offset: usize, count: usize) -> Result<()>;
    fn replace(&mut self, offset: usize, count: usize, data: &str) -> Result<()>;
}

// ------------------------------------------------------------------------------------------------

pub trait Comment: CharacterData {}

// ------------------------------------------------------------------------------------------------

pub trait Document: Node {
    fn create_attribute(&self, name: &str) -> Result<RefNode>;
    fn create_attribute_with(&self, name: &str, value: &str) -> Result<RefNode>;
    fn create_attribute_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode>;
    fn create_cdata_section(&self, data: &str) -> Result<RefNode>;
    fn create_comment(&self, data: &str) -> RefNode;
    fn create_element(&self, tag_name: &str) -> Result<RefNode>;
    fn create_element_ns(&self, namespace_uri: &str, qualified_name: &str) -> Result<RefNode>;
    fn create_processing_instruction(&self, target: &str, data: Option<&str>) -> Result<RefNode>;
    fn create_text_node(&self, data: &str) -> RefNode;
    fn get_element_by_id(&self, id: &str) -> Option<RefNode>;
    fn get_elements_by_tag_name(&self, tag_name: &str) -> Vec<RefNode>;
    fn get_elements_by_tag_name_ns(&self, namespace_uri: &str, local_name: &str) -> Vec<RefNode>;
    fn doc_type(&self) -> Option<RefNode>;
    fn document_element(&self) -> Option<RefNode>;
    fn implementation(&self) -> &Implementation;
}

// ------------------------------------------------------------------------------------------------

pub trait DocumentType: Node {
    fn public_id(&self) -> Option<String>;
    fn system_id(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

pub trait Element: Node {
    fn get_attribute(&self, name: &str) -> Option<String>;
    fn set_attribute(&mut self, name: &str, value: &str) -> Result<()>;
    fn remove_attribute(&mut self, _name: &str) -> Result<()>;
    fn get_attribute_node(&self, name: &str) -> Option<RefNode>;
    fn set_attribute_node(&mut self, _new_attribute: RefNode) -> Result<RefNode>;
    fn remove_attribute_node(&mut self, _old_attribute: RefNode) -> Result<RefNode>;
    fn get_elements_by_tag_name(&self, _tag_name: &str) -> Vec<RefNode>;
    fn get_attribute_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<String>;
    fn set_attribute_ns(
        &mut self,
        namespace_uri: &str,
        qualified_name: &str,
        value: &str,
    ) -> Result<()>;
    fn remove_attribute_ns(&mut self, _namespace_uri: &str, _local_name: &str) -> Result<()>;
    fn get_attribute_node_ns(&self, _namespace_uri: &str, _local_name: &str) -> Option<RefNode>;
    fn set_attribute_node_ns(&mut self, _new_attribute: RefNode) -> Result<RefNode>;
    fn get_elements_by_tag_name_ns(&self, _namespace_uri: &str, _local_name: &str) -> Vec<RefNode>;
    fn has_attribute(&self, name: &str) -> bool;
    fn has_attribute_ns(&self, namespace_uri: &str, local_name: &str) -> bool;
}

// ------------------------------------------------------------------------------------------------

pub trait Node {
    fn name(&self) -> Name;
    fn node_value(&self) -> Option<String>;
    fn set_node_value(&mut self, value: &str) -> Result<()>;
    fn unset_node_value(&mut self) -> Result<()>;
    fn node_type(&self) -> NodeType;
    fn parent_node(&self) -> Option<RefNode>;
    fn child_nodes(&self) -> Vec<RefNode>;
    fn first_child(&self) -> Option<RefNode>;
    fn last_child(&self) -> Option<RefNode>;
    // previousSibling, nextSibling
    fn attributes(&self) -> HashMap<Name, RefNode>;
    fn owner_document(&self) -> Option<RefNode>;
    fn insert_before(&mut self, _new_child: RefNode, _ref_child: &RefNode) -> Result<RefNode>;
    fn replace_child(&mut self, _new_child: RefNode, _old_child: &RefNode) -> Result<RefNode>;
    fn append_child(&mut self, new_child: RefNode) -> Result<RefNode>;
    fn has_child_nodes(&self) -> bool;
    fn clone_node(&self, _deep: bool) -> Option<RefNode>;
    fn normalize(&mut self);
    fn is_supported(&self, feature: String, version: String) -> bool;
    fn has_attributes(&self) -> bool;
}

// ------------------------------------------------------------------------------------------------

pub trait ProcessingInstruction: Node {
    fn length(&self) -> usize {
        match self.data() {
            None => 0,
            Some(s) => s.len(),
        }
    }
    fn data(&self) -> Option<String> {
        Node::node_value(self)
    }
    fn set_data(&mut self, data: &str) -> Result<()> {
        Node::set_node_value(self, data)
    }
    fn unset_data(&mut self) -> Result<()> {
        Node::unset_node_value(self)
    }
    fn target(&self) -> String {
        Node::name(self).to_string()
    }
}

// ------------------------------------------------------------------------------------------------

pub trait Text: CharacterData {
    fn split(_offset: usize) -> Result<RefNode>;
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type RefNode = RcRefCell<NodeImpl>;

pub type WeakRefNode = WeakRefCell<NodeImpl>;

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NodeType {
    Element = 1,
    Attribute,
    Text,
    CData,
    EntityReference,
    Entity,
    ProcessingInstruction,
    Comment,
    Document,
    DocumentType,
    DocumentFragment,
    Notation,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct NodeImpl {
    pub(crate) i_node_type: NodeType,
    pub(crate) i_name: Name,
    pub(crate) i_value: Option<String>,
    pub(crate) i_parent_node: Option<RefNode>,
    pub(crate) i_owner_document: Option<RefNode>,
    pub(crate) i_attributes: HashMap<Name, RefNode>,
    pub(crate) i_child_nodes: Vec<RefNode>,
    // for Document
    pub(crate) i_document_element: Option<RefNode>,
    pub(crate) i_document_type: Option<RefNode>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Implementation {}
