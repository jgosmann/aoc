use anyhow::Context;
use inquire::Password;
use secrecy::SecretBox;

pub struct SessionIdStore {
    entry: keyring::Entry,
}

impl SessionIdStore {
    pub fn new() -> anyhow::Result<Self> {
        let entry = keyring::Entry::new("adventofcode", "session_id")?;
        Ok(Self { entry })
    }

    pub fn prompt(&self) -> anyhow::Result<SecretBox<String>> {
        let session_id = Password::new("Your Advent of Code session id:")
            .without_confirmation()
            .prompt()
            .context("password input")?;
        self.entry.set_password(&session_id)?;
        Ok(SecretBox::new(Box::new(session_id)))
    }

    pub fn session_id(&self) -> anyhow::Result<SecretBox<String>> {
        Ok(match self.entry.get_password() {
            Ok(password) => SecretBox::new(Box::new(password)),
            Err(keyring::Error::NoEntry) => self.prompt()?,
            Err(err) => Err(err).context("credential store")?,
        })
    }
}
