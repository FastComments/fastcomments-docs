## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_notification_body | models::UpdateNotificationBody | Yes |  |
| user_id | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_notification Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params = UpdateNotificationParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "notif-12345".to_string(),
        update_notification_body: models::UpdateNotificationBody {
            enabled: true,
            channel: "email".to_string(),
            subject: "Comment flagged on example.com/article/2026/01/13".to_string(),
            recipients: vec!["moderators@acme.com".to_string()],
        },
        user_id: Some("moderator-42".to_string()),
    };
    let response: FlagCommentPublic200Response = update_notification(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
