pub(crate) struct Google {
    username: String,
    password: String,
    hostname: String,
}

impl Google {
    pub(crate) fn new(username: &str, password: &str, hostname: &str) -> Google {
        Google {
            username: username.to_string(),
            password: password.to_string(),
            hostname: hostname.to_string(),
        }
    }

    // https://support.google.com/domains/answer/6147083?authuser=0&hl=en
    pub(crate) fn run(&self, new_ip: &str) -> crate::Result<()> {
        ureq::get(&format!(
            "https://{}:{}@domains.google.com/nic/update",
            &self.username, &self.password
        ))
        .query("hostname", &self.hostname)
        .query("myip", new_ip)
        .call()?;

        Ok(())
    }
}
