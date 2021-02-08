use crate::identifier::Identifier;
use crate::validation::FieldPath;

#[derive(Debug, PartialEq)]
pub struct NbtDocFile {
    pub uses: Vec<(bool, IdentPath)>,
    pub compounds: Vec<(String, CompoundDef)>,
    pub enums: Vec<(String, EnumDef)>,
    pub describes: Vec<(IdentPath, DescribeDef)>,
    pub mods: Vec<String>,
    pub injects: Vec<InjectDef>,
}

#[derive(Debug, PartialEq)]
pub struct InjectDef {
    pub ty: InjectType,
    pub target: IdentPath,
}

#[derive(Debug, PartialEq)]
pub enum InjectType {
    Compound(Vec<(String, Field)>),
    Enum(EnumType),
}

#[derive(Debug, PartialEq)]
pub struct CompoundDef {
    pub description: String,
    pub fields: Vec<(String, Field)>,
    pub extend: Option<CompoundSuper>,
}

#[derive(Debug, PartialEq)]
pub enum CompoundSuper {
    Compound(IdentPath),
    Registry {
        target: Identifier,
        path: Vec<FieldPath>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub description: String,
    pub optional: bool,
    pub field_type: FieldType,
}

#[derive(Debug, PartialEq)]
pub enum Range<T> {
    Single(T),
    Low(T),
    High(T),
    Both(T, T),
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
    BooleanType,
    NumberType(NumberPrimitiveType),
    StringType,
    ArrayType(NumberArrayType),
    ListType {
        item_type: Box<FieldType>,
        len_range: Option<Range<i32>>,
    },
    NamedType(IdentPath),
    IndexType {
        target: Identifier,
        path: Vec<FieldPath>,
    },
    IdType(Identifier),
    OrType(Vec<FieldType>),
}

#[derive(Debug, PartialEq)]
pub enum NumberPrimitiveType {
    Byte(Option<Range<i8>>),
    Short(Option<Range<i16>>),
    Int(Option<Range<i32>>),
    Long(Option<Range<i64>>),
    Float(Option<Range<f32>>),
    Double(Option<Range<f64>>),
}

#[derive(Debug, PartialEq)]
pub enum NumberArrayType {
    Byte {
        value_range: Option<Range<i8>>,
        len_range: Option<Range<i32>>,
    },
    Int {
        value_range: Option<Range<i32>>,
        len_range: Option<Range<i32>>,
    },
    Long {
        value_range: Option<Range<i64>>,
        len_range: Option<Range<i32>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum EnumType {
    Byte(Vec<(String, EnumValue<i8>)>),
    Short(Vec<(String, EnumValue<i16>)>),
    Int(Vec<(String, EnumValue<i32>)>),
    Long(Vec<(String, EnumValue<i64>)>),
    Float(Vec<(String, EnumValue<f32>)>),
    Double(Vec<(String, EnumValue<f64>)>),
    String(Vec<(String, EnumValue<String>)>),
}

#[derive(Debug, PartialEq)]
pub struct EnumDef {
    pub description: String,
    pub values: EnumType,
}

#[derive(Debug, PartialEq)]
pub struct EnumValue<T> {
    pub description: String,
    pub value: T,
}

#[derive(Debug, PartialEq)]
pub struct DescribeDef {
    pub describe_type: Identifier,
    pub targets: Option<Vec<Identifier>>,
}

type IdentPath = Vec<PathPart>;

#[derive(Debug, PartialEq)]
pub enum PathPart {
    Root,
    Super,
    Regular(String),
}
