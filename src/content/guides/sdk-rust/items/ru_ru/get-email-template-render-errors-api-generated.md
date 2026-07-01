## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| skip | f64 | Нет |  |

## Response

Возвращает: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## Example

[inline-code-attrs-start title = 'Пример get_email_template_render_errors'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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