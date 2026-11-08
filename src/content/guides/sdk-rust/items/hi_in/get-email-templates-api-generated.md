## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_templates_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_email_templates उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetEmailTemplatesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let _response = get_email_templates(&configuration, params).await?;
    Ok(())
}
[inline-code-end]