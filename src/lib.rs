#[derive(Debug, Default, Eq, PartialEq)]
pub struct Path {
    raw_path: String,
}

impl Path {
    pub fn new(path: &str) -> Self {
        Self {
            raw_path: path.to_owned(),
        }
    }

    pub fn new_relative() -> Self {
        Self {
            raw_path: ".".to_owned(),
        }
    }

    pub fn append(mut self, path: &str) -> Self {
        self.raw_path.push('/');
        self.raw_path.push_str(&path);
        self
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&String> for Path {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.raw_path.clone()
    }
}

impl std::ops::Div<&str> for Path {
    type Output = Self;

    fn div(self, rhs: &str) -> Self::Output {
        self.append(rhs)
    }
}

pub trait IntoPath: IntoPathSealed {}

impl IntoPath for str {}

pub trait IntoPathSealed {
    fn into_path(&self) -> Path;
}

impl IntoPathSealed for str {
    fn into_path(&self) -> Path {
        Path::from(self)
    }
}

impl IntoPathSealed for &str {
    fn into_path(&self) -> Path {
        Path::from(*self)
    }
}

impl IntoPathSealed for String {
    fn into_path(&self) -> Path {
        Path::from(self)
    }
}

impl IntoPathSealed for &String {
    fn into_path(&self) -> Path {
        Path::from(*self)
    }
}

#[test]
fn test_append_operator() {
    let result = Path::new_relative() / "Dev" / "Rust" / ".Experiments";
    let expected = Path::new("./Dev/Rust/.Experiments");

    assert_eq!(result, expected);
}

#[test]
fn test_append_function() {
    let result = Path::new_relative()
        .append("Dev")
        .append("Rust")
        .append(".Experiments");

    let expected = Path::new("./Dev/Rust/.Experiments");

    assert_eq!(result, expected);
}
