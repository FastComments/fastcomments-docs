## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| skip | f64 | No |  |

## Response

Returns: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_email_template_render_errors_response.rs)

## Example

[inline-code-attrs-start title = 'get_email_template_render_errors Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetEmailTemplateRenderErrorsParams = GetEmailTemplateRenderErrorsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "welcome-email-v2".to_string(),
    skip: Some(10.0),
};
let response: GetEmailTemplateRenderErrorsResponse = get_email_template_render_errors(&configuration, params).await?;
[inline-code-end]
