use self::super::error::*;
use self::super::name::*;
use self::super::rc_cell::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Attr` interface.
///
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

///
/// This corresponds to the DOM `CDataSection` interface.
///
pub trait CDataSection: Text {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `CharacterData` interface.
///
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

///
/// This corresponds to the DOM `Comment` interface.
///
pub trait Comment: CharacterData {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Document` interface.
///
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

///
/// This corresponds to the DOM `DocumentFragment` interface (current unsupported).
///
pub trait DocumentFragment: Node {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DocumentType` interface.
///
pub trait DocumentType: Node {
    fn public_id(&self) -> Option<String>;
    fn system_id(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Element` interface.
///
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

///
/// This corresponds to the DOM `Entity` interface (currently unsupported).
///
pub trait Entity: Node {
    fn public_id(&self) -> Option<String>;
    fn system_id(&self) -> Option<String>;
    fn notation_name(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `EntityReference` interface (currently unsupported).
///
pub trait EntityReference: Node {}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `Node` interface.
///
/// # Specification
///
/// The Node interface is the primary datatype for the entire Document Object Model. It represents
/// a single node in the document tree. While all objects implementing the Node interface expose
/// methods for dealing with children, not all objects implementing the Node interface may have
/// children. For example, Text nodes may not have children, and adding children to such nodes
/// results in a DOMException being raised.
///
/// The attributes nodeName, nodeValue and attributes are included as a mechanism to get at node
/// information without casting down to the specific derived interface. In cases where there is no
/// obvious mapping of these attributes for a specific nodeType (e.g., nodeValue for an Element or
/// attributes for a Comment), this returns null. Note that the specialized interfaces may contain
/// additional and more convenient mechanisms to get and set the relevant information.
///
/// The values of nodeName, nodeValue, and attributes vary according to the node type as follows:
///
///
/// | Interface               | nodeName                  | nodeValue                           | attributes   |
/// |-------------------------|---------------------------|-------------------------------------|--------------|
/// | `Attr`                  | name of attribute         | value of attribute                  | `None`       |
/// | `CDATASection`          | `"#cdata-section"`        | content of the CDATA Section        | `None`       |
/// | `Comment`               | `"#comment"`              | content of the comment              | `None`       |
/// | `Document`              | `"#document"`             | `None`                              | `None`       |
/// | `DocumentFragment`      | `"#document-fragment"`    | `None`                              | `None`       |
/// | `DocumentType`          | document type name        | `None`                              | `None`       |
/// | `Element`               | tag name                  | `None`                              | `HashMap`    |
/// | `Entity`                | entity name               | `None`                              | `None`       |
/// | `EntityReference`       | name of entity referenced | `None`                              | `None`       |
/// | `Notatio`n              | notation name             | `None`                              | `None`       |
/// | `ProcessingInstruction` | `target`                  | entire content excluding the target | `None`       |
/// | `Text`                  | `"#text"`                 | content of the text node            | `None`       |
///
pub trait Node {
    ///
    /// The name of this node, depending on its type; see the table above.
    ///
    fn name(&self) -> Name;
    ///
    /// The value of this node, depending on its type; see the table above. When it is defined to
    /// be `None`, setting it has no effect.
    ///
    /// # Specification
    ///
    /// **Exceptions on setting**
    ///
    /// * `NO_MODIFICATION_ALLOWED_ERR`: Raised when the node is readonly.
    ///
    /// **Exceptions on retrieval**
    ///
    /// * `DOMSTRING_SIZE_ERR`: Raised when it would return more characters than fit in a DOMString
    /// variable on the implementation platform.
    ///
    fn node_value(&self) -> Option<String>;
    ///
    /// Set the `value` for the node; see [node_value()](#tymethod.node_value).
    ///
    fn set_node_value(&mut self, value: &str) -> Result<()>;
    ///
    /// Set the `value` for the node to `None`; see [node_value()](#tymethod.node_value).
    ///
    fn unset_node_value(&mut self) -> Result<()>;
    ///
    /// A code representing the type of the underlying object.
    ///
    fn node_type(&self) -> NodeType;
    ///
    /// The parent of this node. All nodes, except `Attr`, `Document`, `DocumentFragment`,
    /// `Entity`, and `Notation` may have a parent. However, if a node has just been created and not
    /// yet added to the tree, or if it has been removed from the tree, this is `None`.
    ///
    fn parent_node(&self) -> Option<RefNode>;
    ///
    /// A `Vec` that contains all children of this node. If there are no children,
    /// this is a `Vec` containing no nodes.
    ///
    fn child_nodes(&self) -> Vec<RefNode>;
    ///
    /// The first child of this node. If there is no such node, this returns `None`.
    ///
    fn first_child(&self) -> Option<RefNode>;
    ///
    /// The last child of this node. If there is no such node, this returns `None`.
    ///
    fn last_child(&self) -> Option<RefNode>;
    ///
    /// The node immediately preceding this node. If there is no such node, this returns `None`.
    ///
    fn previous_sibling(&self) -> Option<RefNode>;
    ///
    /// The node immediately following this node. If there is no such node, this returns `None`.
    ///
    fn next_sibling(&self) -> Option<RefNode>;
    ///
    /// A `HashMap` containing the attributes of this node (if it is an `Element`) or
    /// `None` otherwise.
    ///
    fn attributes(&self) -> HashMap<Name, RefNode>;
    ///
    /// The `Document` object associated with this node. This is also the `Document`
    /// object used to create new nodes. When this node is a `Document` or a `DocumentType` which is
    /// not used with any `Document` yet, this is `None`.
    ///
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

///
/// This corresponds to the DOM `Notation` interface (currently unsupported).
///
pub trait Notation: Node {
    fn public_id(&self) -> Option<String>;
    fn system_id(&self) -> Option<String>;
}

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `ProcessingInstruction` interface.
///
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

///
/// This corresponds to the DOM `Text` interface.
///
pub trait Text: CharacterData {
    fn split(_offset: usize) -> Result<RefNode>;
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `DOMImplementation` interface.
///
#[derive(Debug)]
pub struct Implementation {}

// ------------------------------------------------------------------------------------------------

///
/// Internal DOM tree node owned reference
///
pub type RefNode = RcRefCell<NodeImpl>;

///
/// Internal DOM tree node weak reference
///
pub type WeakRefNode = WeakRefCell<NodeImpl>;

// ------------------------------------------------------------------------------------------------

///
/// This corresponds to the DOM `NodeType` set of constants.
///
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum NodeType {
    /// The node is an [Element](trait.Element.html)
    Element = 1,
    /// The node is an [Attribute](trait.Attribute.html)
    Attribute,
    /// The node is a [Text](trait.Text.html)
    Text,
    /// The node is a [CDataSection](trait.CDataSection.html)
    CData,
    /// The node is an `EntityReference`
    EntityReference,
    /// The node is an `Entity`
    Entity,
    /// The node is a [ProcessingInstruction](trait.ProcessingInstruction.html)
    ProcessingInstruction,
    /// The node is a [Comment](trait.Comment.html)
    Comment,
    /// The node is a [Document](trait.Document.html)
    Document,
    /// The node is a [DocumentType](trait.DocumentType.html)
    DocumentType,
    /// The node is a `DocumentFragment`
    DocumentFragment,
    /// The node is a `Notation`
    Notation,
}

// ------------------------------------------------------------------------------------------------

///
/// Internal container for DOM tree node data and state.
///
#[derive(Clone, Debug)]
pub struct NodeImpl {
    pub(crate) i_node_type: NodeType,
    pub(crate) i_name: Name,
    pub(crate) i_value: Option<String>,
    pub(crate) i_parent_node: Option<WeakRefNode>,
    pub(crate) i_owner_document: Option<WeakRefNode>,
    pub(crate) i_attributes: HashMap<Name, RefNode>,
    pub(crate) i_child_nodes: Vec<RefNode>,
    // for Document
    pub(crate) i_document_element: Option<RefNode>,
    pub(crate) i_document_type: Option<RefNode>,
}
