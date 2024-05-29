pub trait PermController {
    fn check<G: Guest>(extension: &E, method: &Method) -> Result<(), Error>;
}

struct SimplePermController;
impl PermController for SimplePermController {
    fn check<E: Extension>(extension: &E, method: &Method) -> Result<(), Error> {
        unimplemented!()
    }
}
