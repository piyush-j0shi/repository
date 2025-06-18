#[derive(Debug)]
pub enum ContactType {
    Personal,
    Work,
    Other,
}

#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub group: ContactType,
}
