## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_notification_body | models::UpdateNotificationBody | 예 |  |
| user_id | String | 아니오 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'update_notification 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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