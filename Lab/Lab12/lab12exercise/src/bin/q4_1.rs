#[derive(Clone)]
enum Text {
    Plain(String),
    Repeated(String, i32)
}
impl Text {
    fn value(&self) -> String {
        match self {
            Text::Plain(t) => t.clone(),
            Text::Repeated(t,n ) => t.repeat(*n as usize)
        }
    }
}
impl From<&Text> for Box<Text> {
    fn from(t: &Text) -> Box<Text> {
        Box::new(t.clone())
    }
}
impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        &self
    }
}
impl Into<String> for &Text {
    fn into(self) -> String {
        self.value()
    }
}
#[test]
fn test_text_repeated() {
let t1 = Text::Plain("Hi".into());
let t2 = Text::Plain("[+]".into());
let t3 = Text::Repeated(t2.as_ref().into(), 3);
let t4 = Text::Repeated(t3.as_ref().into(), 5);
assert_eq!(t1.value(), "Hi");
assert_eq!(t2.value(), "[+]");
assert_eq!(t3.value(), "[+]".repeat(3));
assert_eq!(t4.value(), "[+]".repeat(15));
}
fn main() {}
