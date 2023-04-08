use log::debug;

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
        let status = ureq::get(&format!(
            "https://{}:{}@domains.google.com/nic/update",
            &self.username, &self.password
        ))
        .query("hostname", &self.hostname)
        .query("myip", new_ip)
        .call()?
        .into_string()?;

        debug!("google runner status: {}", status);

        match status.as_str() {
            "nohost" => Err("The hostname doesn't exist, or doesn't have Dynamic DNS enabled.".into()),
            "badauth" => Err("The username/password combination isn't valid for the specified host.".into()),
            "notfqdn" => Err("The supplied hostname isn't a valid fully-qualified domain name.".into()),
            "badagent" => Err("Your Dynamic DNS client makes bad requests. Ensure the user agent is set in the request.".into()),
            "abuse" => Err("Dynamic DNS access for the hostname has been blocked due to failure to interpret previous responses correctly.".into()),
            "911" => Err("An error happened on our end (google side). Wait 5 minutes and retry.".into()),
            "conflict A" => Err("A custom A or AAAA resource record conflicts with the update. Delete the indicated resource record within the DNS settings page and try the update again.".into()),
            "conflict AAAA" => Err("A custom A or AAAA resource record conflicts with the update. Delete the indicated resource record within the DNS settings page and try the update again.".into()),
            // Success status 
            // 1. good {user’s IP address}
            // 2. nochg {user’s IP address}
            &_ => Ok(()),
        }
    }
}
