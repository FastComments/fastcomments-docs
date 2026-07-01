## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| skip | f64 | Nein |  |

## Antwort

Rückgabe: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_email_template_render_errors Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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