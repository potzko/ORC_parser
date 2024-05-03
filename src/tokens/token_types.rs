#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub var_type: Type,
}

#[derive(Debug)]
pub enum Literal {
    Integer(String),
    Floating(String),
    String(String),
    Boolean(String),
    None(),
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    UnaryMinus(Box<Expr>),
    BinOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    ExprBlock(Vec<Statement>, Box<Expr>),
    Variable(Variable),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum Statement {
    LetStatement {
        name: String,
        var_type: Type,
        value: Expr,
    },
    Nop,
}

#[derive(Debug)]
pub enum PrimitiveType {
    Integer,
    Floating,
    Boolean,
    None,
}

#[derive(Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    BaseType(String),
    CompoundType(Box<CompoundType>),
    Unknown,
}

#[derive(Debug)]
pub struct CompoundType {
    pub base: Type,
    pub subtypes: Vec<Type>,
}
