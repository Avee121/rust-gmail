#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

use std::path::Path;

use async_impl::{send_email::send_email, token::retrieve_token};
use error::Result;
use service_account::ServiceAccount;

/// Types for error handling.
pub mod error;

mod async_impl;
mod common;
mod service_account;

#[cfg(feature = "blocking")]
mod blocking;

/// The `GmailClientBuilder` is the intended way of creating a [`GmailClient`].
#[derive(Debug, Clone)]
pub struct GmailClientBuilder {
    service_account: ServiceAccount,
    send_from_email: String,
    mock_mode: bool,
}

impl<'a> GmailClientBuilder {
    /// Create a new `GmailClientBuilder`.
    /// Will return an error if unable to read & parse the `service_account_path` if, for example, the file does not exist or has an incorrect format.
    pub fn new<S: Into<String>>(service_account_json: &str, send_from_email: S) -> Result<Self> {
        Ok(Self {
            service_account: ServiceAccount::load_from_str(service_account_json)?,
            send_from_email: send_from_email.into(),
            mock_mode: false,
        })
    }

    /// Set "mock mode" which, if set to true, will log print the email instead of sending it.
    pub fn mock_mode(mut self, enabled: bool) -> Self {
        self.mock_mode = enabled;
        self
    }

    /// Build a [`GmailClient`] from this `GmailClientBuilder`.
    /// Note: This function will retrieve an access token from the Google API and as such make an API request.
    pub async fn build(self) -> Result<GmailClient> {
        let token = retrieve_token(&self.service_account, &self.send_from_email).await?;

        Ok(GmailClient {
            send_from_email: self.send_from_email,
            token,
            mock_mode: self.mock_mode,
        })
    }

    /// A blocking alternative to the [`build()`] function.
    #[cfg(feature = "blocking")]
    pub fn build_blocking(self) -> Result<GmailClient> {
        use blocking::token::retrieve_token_blocking;

        let token = retrieve_token_blocking(&self.service_account, &self.send_from_email)?;

        Ok(GmailClient {
            send_from_email: self.send_from_email,
            token,
            mock_mode: self.mock_mode,
        })
    }
}

/// A client ready to send emails through the Gmail API.
#[derive(Debug, Clone)]
pub struct GmailClient {
    send_from_email: String,
    token: String,
    mock_mode: bool,
}

impl GmailClient {
    /// Alias for [`GmailClientBuilder::new()`].
    pub fn builder<S: Into<String>>(
        service_account_json: &str,
        send_from_email: S,
    ) -> Result<GmailClientBuilder> {
        GmailClientBuilder::new(service_account_json, send_from_email)
    }

    /// Send an email to `send_to_email` with the specified `subject` and `content`.
    pub async fn send_email(
        &self,
        send_to_email: &str,
        subject: &str,
        content: &str,
    ) -> Result<()> {
        send_email(
            send_to_email,
            subject,
            content,
            &self.token,
            &self.send_from_email,
            self.mock_mode,
        )
        .await
    }

    /// A blocking alternative to [`send_email()`].
    #[cfg(feature = "blocking")]
    pub fn send_email_blocking(
        &self,
        send_to_email: &str,
        subject: &str,
        content: &str,
    ) -> Result<()> {
        use blocking::send_email::send_email_blocking;

        send_email_blocking(
            send_to_email,
            subject,
            content,
            &self.token,
            &self.send_from_email,
            self.mock_mode,
        )
    }
}
