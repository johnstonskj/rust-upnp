use std::result::Result as StdResult;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Corresponds to the DOM `DomException` type.
///
/// # Specification
///
/// DOM operations only raise exceptions in "exceptional" circumstances, i.e., when an operation is
/// impossible to perform (either for logical reasons, because data is lost, or because the
/// implementation has become unstable). In general, DOM methods return specific error values in
/// ordinary processing situation, such as out-of-bound errors when using `NodeList`.
///
#[derive(Clone, Debug)]
#[repr(u16)]
pub enum Error {
    /// If index or size is negative, or greater than the allowed value
    IndexSize = 1,
    /// If the specified range of text does not fit into a DOMString
    StringSize,
    /// If any node is inserted somewhere it doesn't belong
    HierarchyRequest,
    /// If a node is used in a different document than the one that created it (that doesn't
    /// support it)
    WrongDocument,
    /// If an invalid or illegal character is specified, such as in a name. See production 2 in the
    /// XML specification for the definition of a legal character, and production 5 for the
    /// definition of a legal name character.
    InvalidCharacter,
    /// If data is specified for a node which does not support data
    NoDataAllowed,
    /// If an attempt is made to modify an object where modifications are not allowed
    NoModificationAllowed,
    /// If an attempt is made to reference a node in a context where it does not exist
    NotFound,
    /// If the implementation does not support the requested type of object or operation
    NotSupported,
    /// If an attempt is made to add an attribute that is already in use elsewhere
    InUseAttribute,
    /// If an attempt is made to use an object that is not, or is no longer, usable (introduced in DOM Level 2)
    InvalidState,
    /// If an invalid or illegal string is specified (introduced in DOM Level 2)
    Syntax,
    /// If an attempt is made to modify the type of the underlying object (introduced in DOM
    /// Level 2)
    InvalidModification,
    /// If an attempt is made to create or change an object in a way which is incorrect with
    /// regard to namespaces (introduced in DOM Level 2)
    Namespace,
    /// If a parameter or an operation is not supported by the underlying object (introduced in
    /// DOM Level 2)
    InvalidAccess,
}

///
/// This standard `Result` structure is used wherever an IDL function is marked as throwing
/// exceptions.
///
pub type Result<T> = StdResult<T, Error>;
