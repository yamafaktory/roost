#[derive(Debug)]
pub enum Entity {
    Block(&'static str),
    Traversable(&'static str),
}

impl Entity {
    // Implement a to_string method to get the content of the entities.
    pub fn to_string(&self) -> &'static str {
        return match self {
            &Entity::Block(string) => string,
            &Entity::Traversable(string) => string
        };
    }
}
