#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    path: String,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let path = path.to_string();
        Self { path }
    }

    pub fn components(&self) -> Vec<String> {
        self.path
            .split("/")
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }

    pub fn name(&self) -> Option<String> {
        self.components().last().cloned()
    }

    pub fn parent(&self) -> Self {
        let mut parent = self.clone();

        let mut components = parent.components();

        components.pop();

        match components.len() > 0 {
            true => parent.path = format!("/{}", components.join("/")),
            false => parent.path = "/".to_string(),
        }

        parent
    }

    pub fn to_string(&self) -> String {
        self.path.clone()
    }

    pub fn join<T: Into<Self>>(&self, path: T) -> Self {
        Self::new(&format!("{}{}", self.path, path.into().path))
    }

    pub fn push(&mut self, path: &str) {
        self.path.push_str(&format!("{}/", path));
    }

    pub fn pop(&mut self) {
        let mut components = self.components();
        components.pop();
        match components.len() > 0 {
            true => self.path = format!("/{}/", components.join("/")),
            false => self.path = "/".to_string(),
        }
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            path: String::from("/"),
        }
    }
}

impl Into<Path> for std::path::PathBuf {
    fn into(self) -> Path {
        Path::new(&self.display().to_string())
    }
}

impl Into<Path> for &std::path::Path {
    fn into(self) -> Path {
        Path::new(&self.display().to_string())
    }
}

impl Into<Path> for &str {
    fn into(self) -> Path {
        Path::new(&self)
    }
}

impl Into<Path> for String {
    fn into(self) -> Path {
        Path::new(&self)
    }
}
