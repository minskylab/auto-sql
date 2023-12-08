#[derive(Debug)]
pub struct Representation {
    pub name: String,
    pub scalars: Vec<Scalar>,
    pub relations: Vec<Relation>,
}

#[derive(Debug)]
pub struct Scalar {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

#[derive(Debug)]
pub struct Relation {
    pub name: String,
    pub nullable: bool,
    pub to_table: String,
    pub to_column: String,
}

pub trait Representable {
    fn representation() -> Representation;
}
