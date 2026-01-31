## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_email_template_body | models::UpdateEmailTemplateBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_email_template Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_email_template() -> Result<(), Error> {
    let params = UpdateEmailTemplateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment_notification".to_string(),
        update_email_template_body: models::UpdateEmailTemplateBody {
            subject: "New comment on \"{{post_title}}\"".to_string(),
            html: "<p>{{author_name}} left a comment:</p><blockquote>{{comment_excerpt}}</blockquote>".to_string(),
            enabled: Some(true),
            from_name: Some("Acme News".to_string()),
            reply_to: Some("notifications@acme.example".to_string()),
        },
    };
    let response: FlagCommentPublic200Response = update_email_template(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]
