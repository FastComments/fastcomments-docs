## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| error_id | String | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_email_template_render_error Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteEmailTemplateRenderErrorParams = DeleteEmailTemplateRenderErrorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "welcome-email-template".to_string(),
        error_id: "render-err-2026-0001".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_email_template_render_error(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
