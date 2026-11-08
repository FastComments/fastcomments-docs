## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_email_template_render_errors उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_template_errors(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetEmailTemplateRenderErrorsParams {
        tenant_id: "acme-corp".to_string(),
        id: "newsletter-welcome".to_string(),
        skip: Some(5.0),
    };
    let _response: GetEmailTemplateRenderErrorsResponse = get_email_template_render_errors(config, params).await?;
    Ok(())
}
[inline-code-end]