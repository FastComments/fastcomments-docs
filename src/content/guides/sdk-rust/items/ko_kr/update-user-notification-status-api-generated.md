## Parameters

| 이름 | 타입 | 필요 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| notification_id | String | 예 |  |
| new_status | String | 예 |  |
| sso | String | 아니오 |  |

## Response

반환: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_response.rs)

## Example

[inline-code-attrs-start title = 'update_user_notification_status 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "news/article".to_string(),
        new_status: "read".to_string(),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: UpdateUserNotificationStatusResponse =
        update_user_notification_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---