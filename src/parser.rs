mod tokenizer;
/// OpArity is an enumeration that represents the four ways identifiers and operators can be used.
///
///  **Binary**: Two operands, on the left and right. This is only macros for Alnozo.
///  **Literal**: A literal, atomic value.
///  **Unary**: An operator that has only one operand, on the right.
///  **Funciton**: Either a macro or a function called normally. **TODO**: Rename this.
pub enum OpArity { Binary, Literal, Unary, Function }

/// Node is a struct, that represents a node in a tree representing the parsing structure.
///
/// Nodes are recursive, becouse the parse of a source file will represent a tree. In Node, if it's
/// arity is binary, both the first and second properties will have "Some"thing in them. Otherwise,
/// one or the other will have another node. That is why it is a boxed option node. The value field
/// will contain a representation of the item, as a string. Becouse numbers are not supported in
/// Alonzo, we have avoided the problem of representing both a number and a string in a Rust
/// struct.
pub struct Node {
    pub arity: OpArity,
    pub value: String,
    pub first: Option<Box<Node>>,
    pub second: Option<Box<Node>>
}
fn parse(tokens: Vec<tokenizer::Token>) -> Node {
    
}
