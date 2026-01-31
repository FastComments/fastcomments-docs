## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_email_template Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let tenant_id: Option<String> = Some("acme-corp-tenant".to_string());
    let template_id: Option<String> = Some("welcome-email-template-42".to_string());
    let params: DeleteEmailTemplateParams = DeleteEmailTemplateParams {
        tenant_id: tenant_id.unwrap(),
        id: template_id.unwrap(),
    };
    let response: FlagCommentPublic200Response = delete_email_template(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
