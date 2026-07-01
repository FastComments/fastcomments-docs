## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|------|------|
| tenant_id | String | はい |  |
| id | String | はい |  |

## レスポンス

戻り値: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_cached_notification_count_response.rs)

## 例

[inline-code-attrs-start title = 'get_cached_notification_count の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notification_count() -> Result<(), Error> {
    let params = GetCachedNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
    };
    let response = get_cached_notification_count(&configuration, params).await?;
    let _ = response.user_notification_count;
    Ok(())
}
[inline-code-end]