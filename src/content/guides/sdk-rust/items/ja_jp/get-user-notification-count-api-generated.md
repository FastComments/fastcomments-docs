---
## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## 例

[inline-code-attrs-start title = 'get_user_notification_count の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let response = get_user_notification_count(&config, params).await?;
    println!("{:?}", response);
    Ok(())
}
[inline-code-end]

---