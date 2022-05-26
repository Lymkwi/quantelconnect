extern crate tini;

use tini::Ini;

pub struct Values {
    user: String,
    pass: String,
    force: bool
}

impl Values {
    pub fn get_user(&self) -> &str { &self.user }
    pub fn get_pass(&self) -> &str { &self.pass }
    pub const fn get_force(&self) -> bool { self.force }
}

// Builder paradigm
pub struct QBuilder {
    user: Option<String>,
    pass: Option<String>,
    force: bool
}

impl QBuilder {
    pub fn build(self) -> Result<Values, String> {
        match self.user {
            Some(u) => match self.pass {
                Some(p) => Ok(Values { user: u, pass: p, force: self.force }),
                None => Err("Missing password. Aborting.".into())
            },
            None => Err("Missing user name. Aborting.".into())
        }
    }
    pub fn read_file(mut self, file: Option<&str>)
        -> Result<Self, Box<dyn std::error::Error>> {
        // Do the tini
        match file {
            Some(fname) => {
                let conffile = Ini::from_file(fname)?;
                self.user = conffile.get("quantic", "user");
                self.pass = conffile.get("quantic", "password");
                self.force = match conffile.get("quantic", "force") {
                    Some(v) => v,
                    None => self.force
                };
                Ok(self)
            },
            None => Ok(self)
        }
    }
    pub fn set_user(mut self, user: Option<&str>) -> Self {
        if let Some(u) = user {
            self.user = Some(u.into());
        }
        self
    }
    pub fn set_pass(mut self, pass: Option<&str>) -> Self {
        if let Some(p) = pass {
            self.pass = Some(p.into());
        }
        self
    }
    pub const fn set_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }
    pub const fn new() -> Self {
        Self { user: None, pass: None, force: false }
    }
}

impl std::fmt::Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Config(user={}, pass={}, force={})",
            self.user, self.pass, self.force)
    }
}
