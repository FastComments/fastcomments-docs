## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| from_name | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'send_invite उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = SendInviteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        from_name: "John Doe".to_string(),
        message: Some("Welcome to the platform".to_string()),
        ..Default::default()
    };
    let _ = send_invite(configuration, params).await?;
    Ok(())
}
[inline-code-end]