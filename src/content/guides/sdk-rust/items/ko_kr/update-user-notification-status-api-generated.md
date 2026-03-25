## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| notification_id | String | 예 |  |
| new_status | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## 예제

[inline-code-attrs-start title = 'update_user_notification_status 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params: UpdateUserNotificationStatusParams = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "notif-2026-04-01-7f3b".to_string(),
        new_status: "read".to_string(),
        sso: Some("sso-session-abcdef123456".to_string()),
    };
    let resp: UpdateUserNotificationStatus200Response =
        update_user_notification_status(&configuration, params).await?;
    Ok(resp)
}
[inline-code-end]

---