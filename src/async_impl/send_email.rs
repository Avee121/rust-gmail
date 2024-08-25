use crate::{
    common::send_email::{
        mock_print_email, GoogleSendEmailRequest, GoogleSendEmailResponse, SEND_EMAIL_ENDPOINT,
        SEND_EMAIL_QUERY_PARAMETERS,
    },
    error::{GoogleApiError, Result},
};

pub async fn send_email(
    receiver_email: &str,
    subject: &str,
    content: &str,
    token: &str,
    send_from_email: &str,
    timeout: Option<std::time::Duration>,
    mock_mode: bool,
) -> Result<()> {
    if mock_mode {
        mock_print_email(receiver_email, subject, content, send_from_email);
        return Ok(());
    }

    do_send_email(
        receiver_email,
        subject,
        content,
        token,
        send_from_email,
        timeout,
    )
    .await
}

async fn do_send_email(
    receiver_email: &str,
    subject: &str,
    content: &str,
    token: &str,
    send_from_email: &str,
    timeout: Option<std::time::Duration>,
) -> Result<()> {
    let send_email_request =
        GoogleSendEmailRequest::new(send_from_email, receiver_email, subject, content);

    let mut client = reqwest::Client::builder();

    if let Some(timeout) = timeout {
        client = client.timeout(timeout);
    }

    let response_text = client
        .build()?
        .post(SEND_EMAIL_ENDPOINT)
        .query(&SEND_EMAIL_QUERY_PARAMETERS)
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .header(r"contentType", "text/html")
        .json(&send_email_request)
        .send()
        .await?
        .text()
        .await?;

    let _response: GoogleSendEmailResponse = serde_json::from_str(&response_text)
        .map_err(|_| GoogleApiError::EmailSendError(response_text))?;

    Ok(())
}
