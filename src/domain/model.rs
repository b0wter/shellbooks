use crate::domain::library::Library;

pub struct Model<'a> {
    pub library: &'a Library,
}
