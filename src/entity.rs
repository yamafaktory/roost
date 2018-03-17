#[derive(Debug)]
pub enum Entity {
    Block(&'static str),
    Traversable(&'static str),
}

impl Entity {
    // Implement a to_string method to get the content of the entities.
    pub fn to_string(&self) -> &'static str {
        match *self {
            Entity::Block(string) | Entity::Traversable(string) => string,
        }
    }
    pub fn is_traversable(&self) -> bool {
        match *self {
            Entity::Block(_) => false,
            Entity::Traversable(_) => true,
        }
    }
}
