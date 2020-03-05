/*!
A reasonably faithful implementation of the W3C [Document Object Model Core, Level
2](https://www.w3.org/TR/DOM-Level-2-Core), built on top of the
[quick_xml](https://docs.rs/quick-xml) crate.

# Example

TBD

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

Rather than dealing with a lot of traits which may, or may not, be implemented by a common
concrete type, this crate provides concrete types for a number of the core interfaces and instead
of downcasting `Node` to `Element` say, the `Node` struct supports a set of `is_{node-type}`
predicates and `as_{node-type}` functions.

Wherever possible the documentation included with this module is taken from the specification
documents listed above.

## Interface Mapping

| IDL Interface           | Rust Mapping                                                |
|-------------------------|-------------------------------------------------------------|
| `Attr`                  | [Attribute](struct.Attribute.html)                          |
| _`CharacterData`_       | [CharacterData](struct.CharacterData.html)                  |
| `CDATASection`          | See _CharacterData_                                         |
| `Comment`               | _CharacterData_                                             |
| `Document`              | [Document](struct.Document.html)                            |
| `DocumentFragment`      | Not Supported                                               |
| `DocumentType`          | [DocumentType](struct.DocumentType.html)                    |
| `DOMImplementation`     | Not Supported                                               |
| `Element`               | [Element](struct.Element.html)                              |
| `Entity`                | Not Supported                                               |
| `EntityReference`       | Not Supported                                               |
| `NamedNodeMap`          | `HashMap<String, Weak<NodeImpl>`                            |
| `Node`                  | [Node](structNode..html)                                    |
| `NodeList`              | `Vec<Rc<NodeImpl>>`                                         |
| `Notation`              | Not Supported                                               |
| `ProcessingInstruction` | [ProcessingInstruction](struct.ProcessingInstruction.html)  |
| `Text`                  | _CharacterData_                                             |

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
