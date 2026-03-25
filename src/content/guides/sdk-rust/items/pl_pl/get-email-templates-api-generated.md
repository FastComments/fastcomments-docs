## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| skip | f64 | Nie |  |

## Odpowiedź

Zwraca: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_templates_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_email_templates Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_email_templates() -> Result<GetEmailTemplates200Response, Error> {
    let params: GetEmailTemplatesParams = GetEmailTemplatesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let templates: GetEmailTemplates200Response = get_email_templates(&configuration, params).await?;
    Ok(templates)
}
[inline-code-end]