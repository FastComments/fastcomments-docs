## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니오 |  |
| after_created_at | i64 | 아니오 |  |
| unread_only | bool | 아니오 |  |
| dm_only | bool | 아니오 |  |
| no_dm | bool | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## 예제

[inline-code-attrs-start title = 'reset_user_notifications 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_reset() -> Result<(), Error> {
    let params: ResetUserNotificationsParams = ResetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("notif-20260619-0001".to_string()),
        after_created_at: Some(1_787_400_000i64),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(false),
        sso: Some("saml".to_string()),
    };
    let response: ResetUserNotificationsResponse =
        reset_user_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---