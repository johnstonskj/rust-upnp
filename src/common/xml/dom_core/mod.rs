/*!
A reasonably faithful implementation of the W3C [Document Object Model Core, Level
2](https://www.w3.org/TR/DOM-Level-2-Core), built on top of the
[quick_xml](https://docs.rs/quick-xml) crate.

# Example

```rust
use upnp_rs::dom_core::*;

let mut document_node =
    Implementation::create_document("uri:urn:simons:thing:1", "root", None).unwrap();

let document = &document_node as &dyn Document;
let root = document.create_element("root").unwrap();

let mut root_node = document_node.append_child(root).unwrap();
let root = &mut root_node as &mut dyn Element;
root.set_attribute("version", "1.0");
root.set_attribute("something", "else");

let xml = document_node.to_string();
println!("document 2: {}", xml);
```

# Specification

* [Document Object Model (DOM) Level 1 Specification](https://www.w3.org/TR/REC-DOM-Level-1/),
   Version 1.0, W3C Recommendation 1 October, 1998. Specifically §1, _Document Object Model (Core)
   Level 1_.
* [Document Object Model (DOM) Level 2 Core Specification](https://www.w3.org/TR/DOM-Level-2-Core/),
   Version 1.0, W3C Recommendation 13 November, 2000. Specifically §1, _Document Object Model Core_.

# IDL to Rust Mapping

From the core documentation:

> The `Node` interface is the primary datatype for the entire Document Object Model. It represents
> a single node in the document tree. While all objects implementing the `Node` interface expose
> methods for dealing with children, not all objects implementing the `Node` interface may have
> children. For example, `Text` nodes may not have children, and adding children to such nodes
> results in a DOMException being raised.

> The attributes `nodeName`, `nodeValue` and `attributes` are included as a mechanism to get at
> node information without casting down to the specific derived interface. In cases where there is
> no obvious mapping of these attributes for a specific `nodeType` (e.g., `nodeValue` for an
> `Element` or `attributes` for a `Comment`), this returns `null`. Note that the specialized
> interfaces may contain additional and more convenient mechanisms to get and set the relevant
> information.



Wherever possible the documentation included in sections headed **Specification**  is taken from
the specification documents listed above.

## Interface Mapping

The actual concrete types used in the DOM tree are [RefNode](type.RefNode.html)
and [WeakRefNode](type.WeakRefNode.html) which in turn are references to the opaque
[NodeImpl](struct.NodeImpl.html) struct. Only `RefNode` implements all of the DOM interfaces
and in general the programmer should never need to interact with `WeakRefNode`.

| IDL Interface           | Rust Mapping                                                |
|-------------------------|-------------------------------------------------------------|
| `Attr`                  | [Attribute](trait.Attribute.html)                           |
| _`CharacterData`_       | [CharacterData](trait.CharacterData.html)                   |
| `CDATASection`          | [CDataSection](trait.CDataSection.html)                     |
| `Comment`               | [Document](trait.Document.html)                             |
| `Document`              | [Document](trait.Document.html)                             |
| `DocumentFragment`      | Not Supported                                               |
| `DocumentType`          | [DocumentType](trait.DocumentType.html)                     |
| `DOMImplementation`     | [Implementation](struct.Implementation.html)                |
| `Element`               | [Element](trait.Element.html)                               |
| `Entity`                | Not Supported                                               |
| `EntityReference`       | Not Supported                                               |
| `NamedNodeMap`          | `HashMap<Name, RefNode>`                                    |
| `Node`                  | [Node](trait.Node.html)                                     |
| `NodeList`              | `Vec<Rc<RefNode>>`                                          |
| `Notation`              | Not Supported                                               |
| `ProcessingInstruction` | [ProcessingInstruction](struct.ProcessingInstruction.html)  |
| `Text`                  | [Text](trait.Text.html)                                     |

* The exception type `DOMException` and associated constants are represented by the enumeration
  `Error`.
* IDL Interface attributes are represented by functions;
  * readonly attributes simply have an `attribute_name` getter,
  * writeable attributes also have a `set_attribute_name` setter,
  * some attributes allow null in which case they have an `unset_attribute_name` setter.
* IDL function names are altered from `lowerCamelCase` to `snake_case`.
* IDL functions that are marked `raises(DOMException)` return [`Result`](type.Result.html) with
  [`Error`](enum.Error.html) as the error type.
* IDL attributes of type `T` that are described as "_may be `null`_", or IDL functions that "_may
  return `T` or `null`_" instead return `Option<T>`.

## Primitive Type Mapping

| IDL Type         | Rust Type        | Usage                                |
|------------------|------------------|--------------------------------------|
| `boolean`        | `bool`           | all                                  |
| `DOMString`      | `String`         | all                                  |
| `unsigned short` | `Error`, `u16`   | as representation of exception code  |
| `unsigned long`  | `usize`          | list/string indexes and lengths      |

## Ownership

The field `children` on `Document` and `NodeImpl` own the nodes of the DOM tree using `Rc`. Other
references to children, for example the `document_element` or `attributes` use `Weak` references.
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod error;
pub use error::*;

mod tests;
pub use tests::*;

mod name;
pub use name::*;

mod rc_cell;

mod traits;
pub use traits::*;

mod trait_impls;
pub use trait_impls::*;

pub(crate) mod syntax;
