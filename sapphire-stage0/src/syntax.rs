pub mod decl;
pub type Integer = i32;
pub type Float = f32;
pub type Identifier = String;
pub type TypeIdentifier = String;

#[derive(PartialEq, Debug)]
pub enum Literal {
  Integer(Integer),
  Float(Float),
  String(String),
}

#[derive(PartialEq, Debug)]
pub struct Block {
  pub expr_list: Vec<PrimaryExpression>,
}

#[derive(PartialEq, Debug)]
pub struct Call {
  pub post_expr: Box<PostfixExpression>,
  pub arg_list: Vec<PrimaryExpression>,
}

#[derive(PartialEq, Debug)]
pub struct SequenceAccess {
  pub post_expr: Box<PostfixExpression>,
  pub access_expr: Box<PostfixExpression>,
}

#[derive(PartialEq, Debug)]
pub struct NavigationAccess {
  pub post_expr: Box<PostfixExpression>,
  pub id: Identifier,
}

#[derive(PartialEq, Debug)]
pub struct VariableDeclaration {
  pub id: Identifier,
  pub type_id: Option<Identifier>,
  pub init_expr: Option<PostfixExpression>,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
  Block(Block),
}

#[derive(PartialEq, Debug)]
pub enum PostfixExpression {
  Primary(PrimaryExpression),
  Call(Call),
  SeqAccess(SequenceAccess),
  NavigationAccess(NavigationAccess),
}

#[derive(PartialEq, Debug)]
pub enum PrimaryExpression {
  Literal(Literal),
  Identifier(Identifier),
}
