## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_notification_body | models::UpdateNotificationBody | 是 |  |
| user_id | String | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'update_notification 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run_update_notification(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateNotificationParams = UpdateNotificationParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "notification-67890".to_string(),
        update_notification_body: models::UpdateNotificationBody {
            title: Some("Flag Review Complete".to_string()),
            message: Some("A moderator reviewed the flagged comment and marked it resolved.".to_string()),
            resolved: Some(true),
            channels: Some(vec!["email".to_string(), "in_app".to_string()]),
        },
        user_id: Some("moderator-007".to_string()),
    };
    let response: FlagCommentPublic200Response = update_notification(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---