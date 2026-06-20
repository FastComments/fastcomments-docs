## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| after_id | String | いいえ |  |
| after_created_at | i64 | いいえ |  |
| unread_only | bool | いいえ |  |
| dm_only | bool | いいえ |  |
| no_dm | bool | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## 例

[inline-code-attrs-start title = 'reset_user_notifications の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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