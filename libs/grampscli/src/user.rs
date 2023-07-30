pub struct User {}

impl User {
    pub fn new() -> Self {
        log::trace!("Creating User ...");
        Self {}
    }
}
