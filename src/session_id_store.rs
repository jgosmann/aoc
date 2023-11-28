use anyhow::Context;
use inquire::Password;
use secrecy::Secret;

pub struct SessionIdStore {
    entry: keyring::Entry,
}

impl SessionIdStore {
    pub fn new() -> anyhow::Result<Self> {
        let entry = keyring::Entry::new("adventofcode", "session_id")?;
        Ok(Self { entry })
    }

    pub fn prompt(&self) -> anyhow::Result<Secret<String>> {
        let session_id = Password::new("Your Advent of Code session id:")
            .without_confirmation()
            .prompt()
            .context("password input")?;
        self.entry.set_password(&session_id)?;
        Ok(Secret::new(session_id))
    }

    pub fn session_id(&self) -> anyhow::Result<Secret<String>> {
        Ok(match self.entry.get_password() {
            Ok(password) => Secret::new(password),
            Err(keyring::Error::NoEntry) => self.prompt()?,
            Err(err) => Err(err).context("credential store")?,
        })
    }
}
