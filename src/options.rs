#[derive(Clone)]
pub struct ArcadeDBOptions {
    pub url: String,
    pub auth: Auth,
}

impl Default for ArcadeDBOptions {
    fn default() -> Self {
        Self {
            url: String::from("http://localhost:2480"),
            auth: Auth::NoAuth,
        }
    }
}

#[derive(Clone)]
pub enum Auth {
    NoAuth,
    Basic(Credentials),
}

impl Auth {
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Auth {
        Auth::Basic(Credentials {
            username: username.into(),
            password: password.into(),
        })
    }
}

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
