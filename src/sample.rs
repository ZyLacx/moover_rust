struct MyStruct {
    niečo: String,
}
impl MyStruct {
    fn add(&mut self) {
        self.niečo.push(char::from_digit(2, 2).unwrap());
    }
}
trait Countable {
    fn count(&self) -> usize;
}
impl Countable for MyStruct {
    fn count(&self) -> usize {
        self.niečo.len()
    }
}
fn smt(var: Box<dyn Countable>) {
    var.count();
}