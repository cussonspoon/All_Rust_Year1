trait Text {
    fn value(&self) -> String;
    fn clone_box(&self) -> Box<dyn Text>;
}

impl Clone for Box<dyn Text> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct PlainText {
    chars: String,
}

impl From<&str> for PlainText {
    fn from(text: &str) -> PlainText {
        PlainText { chars: text.to_string() }
    }
}

impl Text for PlainText {
    fn value(&self) -> String {
        self.chars.clone()
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for PlainText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

#[derive(Clone)]
struct RepeatedText {
    text: Box<dyn Text>,
    count: usize,
}

impl RepeatedText {
    fn with_parts(text: &dyn Text, count: usize) -> RepeatedText {
        RepeatedText {
            text: Box::new(text.clone_box()),
            count,
        }
    }
}

impl Text for RepeatedText {
    fn value(&self) -> String {
        self.text.value().repeat(self.count)
    }

    fn clone_box(&self) -> Box<dyn Text> {
        Box::new(self.clone())
    }
}

impl AsRef<dyn Text> for RepeatedText {
    fn as_ref(&self) -> &(dyn Text + 'static) {
        self
    }
}

impl Text for Box<dyn Text> {
    fn value(&self) -> String {
        self.as_ref().value()
    }

    fn clone_box(&self) -> Box<dyn Text> {
        self.as_ref().clone_box()
    }
}

#[test]
fn test_text_repeated2() {
    let t1 = PlainText::from("Hi");
    let t2 = PlainText::from("[+]");
    let t3 = RepeatedText::with_parts(&t2, 3);
    let t4 = RepeatedText::with_parts(&t3, 5);
    assert_eq!(t1.value(), "Hi");
    assert_eq!(t2.value(), "[+]");
    assert_eq!(t3.value(), "[+]".repeat(3));
    assert_eq!(t4.value(), "[+]".repeat(15));
}

fn main(){}