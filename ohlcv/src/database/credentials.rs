use slugify::slugify;

/// Credentials for the database.
#[derive(Debug, PartialEq, Eq)]
pub struct Credentials {
    username: String,
    password: Option<String>,
}

impl Credentials {
    /// Create new credentials with the specified username.
    ///
    /// The password is looked up in the environment variable
    /// `OHLCV_<username>_PASSWORD`. The username is slugified with underscores
    /// and converted to uppercase. If the password is not found in the
    /// environment variable, it is not set and may be set manually.
    ///
    /// To set the password manually, use the
    /// [`with_password()`](Self::with_password) method.
    #[must_use]
    pub fn new(username: impl Into<String>) -> Self {
        let username = username.into();
        let envar = slugify!(&username, separator = "_").to_uppercase();
        let envar = format!("OHLCV_{envar}_PASSWORD");
        let password = std::env::var(envar).ok();

        Self { username, password }
    }

    /// Set the password for the credentials.
    #[must_use]
    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Get the username for the credentials.
    #[inline]
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the password for the credentials.
    #[inline]
    #[must_use]
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    /// Checks if the password is set.
    #[inline]
    #[must_use]
    pub const fn has_password(&self) -> bool {
        self.password.is_some()
    }
}

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
impl TryFrom<&crate::database::mysql::DbConfig> for Credentials {
    type Error = crate::Error;

    /// Convert the database configuration into credentials.
    ///
    /// If the password is set in the configuration, it is used. Otherwise, the
    /// password is looked up in the environment variable
    /// `OHLCV_<username>_PASSWORD`.
    ///
    /// # Errors
    ///
    /// Returns an error if the password is missing.
    fn try_from(config: &crate::database::mysql::DbConfig) -> Result<Self, Self::Error> {
        let creds = Self::new(&config.username);

        if let Some(password) = &config.password {
            Ok(creds.with_password(password))
        } else if creds.has_password() {
            Ok(creds)
        } else {
            Err(Self::Error::MissingPassword(creds.username().into()))
        }
    }
}

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
impl TryFrom<&crate::database::postgres::DbConfig> for Credentials {
    type Error = crate::Error;

    /// Convert the database configuration into credentials.
    ///
    /// If the password is set in the configuration, it is used. Otherwise, the
    /// password is looked up in the environment variable
    /// `OHLCV_<username>_PASSWORD`.
    ///
    /// # Errors
    ///
    /// Returns an error if the password is missing.
    fn try_from(config: &crate::database::postgres::DbConfig) -> Result<Self, Self::Error> {
        let creds = Self::new(&config.username);

        if let Some(password) = &config.password {
            Ok(creds.with_password(password))
        } else if creds.has_password() {
            Ok(creds)
        } else {
            Err(Self::Error::MissingPassword(creds.username().into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const USERNAMES: &[(&str, &str)] = &[
        ("test", "OHLCV_TEST_PASSWORD"),
        ("test_user", "OHLCV_TEST_USER_PASSWORD"),
        ("test-user", "OHLCV_TEST_USER_PASSWORD"),
        ("test_user_1", "OHLCV_TEST_USER_1_PASSWORD"),
        ("test-user-1", "OHLCV_TEST_USER_1_PASSWORD"),
        ("test-üser-1", "OHLCV_TEST_USER_1_PASSWORD"),
    ];

    #[test]
    fn new() {
        let creds = Credentials::new("test");
        assert_eq!(creds.username(), "test");
        assert!(!creds.has_password());

        for (username, envar) in USERNAMES {
            std::env::set_var(envar, "password");
            let creds = Credentials::new(*username);
            assert_eq!(creds.username(), *username);
            assert_eq!(creds.password(), Some("password"));
            std::env::remove_var(envar);
        }
    }

    #[test]
    fn with_password() {
        let envar = "OHLCV_TEST_PASSWORD";
        std::env::set_var(envar, "password2");

        let creds = Credentials::new("test").with_password("password");
        assert_eq!(creds.username(), "test");
        assert_eq!(creds.password(), Some("password"));
        std::env::remove_var(envar);
    }

    #[cfg(feature = "mysql")]
    #[test]
    fn from_mysql() {
        let envar = "OHLCV_TEST_PASSWORD";
        std::env::set_var(envar, "password2");

        let config = crate::database::mysql::DbConfig {
            host: "localhost".into(),
            port: Some(3306),
            database: "test".into(),
            username: "test".into(),
            password: Some("password".into()),
            root_username: None,
            pool: None,
        };

        let creds = Credentials::try_from(&config);
        assert_eq!(
            creds,
            Ok(Credentials::new("test").with_password("password"))
        );

        let config = crate::database::mysql::DbConfig {
            host: "localhost".into(),
            port: Some(3306),
            database: "test".into(),
            username: "test".into(),
            password: None,
            root_username: None,
            pool: None,
        };

        let creds = Credentials::try_from(&config);
        assert_eq!(
            creds,
            Ok(Credentials::new("test").with_password("password2"))
        );
        std::env::remove_var(envar);
    }
}
