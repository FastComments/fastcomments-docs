## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| render_email_template_body | models::RenderEmailTemplateBody | Yes |  |
| locale | String | No |  |

## Response

Returns: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/render_email_template_200_response.rs)

## Example

[inline-code-attrs-start title = 'render_email_template Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<RenderEmailTemplate200Response, Error> {
    let params: RenderEmailTemplateParams = RenderEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        render_email_template_body: models::RenderEmailTemplateBody {
            template_id: "weekly-digest".to_string(),
            subject: "Weekly News Digest".to_string(),
            html: "<h1>Weekly News</h1><p>Top stories and updates</p>".to_string(),
            data: std::collections::HashMap::from([("article_path".to_string(), "news/article".to_string())]),
        },
        locale: Some("en-US".to_string()),
    };
    let response: RenderEmailTemplate200Response = render_email_template(configuration, params).await?;
    Ok(response)
}
[inline-code-end]
