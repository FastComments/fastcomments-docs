## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| skip | f64 | Nee |  |

## Response

Retourneert: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_email_template_render_errors Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let params: GetEmailTemplateRenderErrorsParams = GetEmailTemplateRenderErrorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
        skip: Some(10.0),
    };
    let response: GetEmailTemplateRenderErrors200Response =
        get_email_template_render_errors(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---