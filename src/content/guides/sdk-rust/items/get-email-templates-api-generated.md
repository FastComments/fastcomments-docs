## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| skip | f64 | No |  |

## Response

Returns: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_templates_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_email_templates Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetEmailTemplatesParams = GetEmailTemplatesParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        skip: Some(10.0),
    };
    let templates: GetEmailTemplates200Response = get_email_templates(&configuration, params).await?;
    let _ = templates;
    Ok(())
}
[inline-code-end]
