## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| user_id | String | いいえ |  |
| url_id | String | いいえ |  |
| from_comment_id | String | いいえ |  |
| viewed | bool | いいえ |  |
| skip | f64 | いいえ |  |

## レスポンス

返却: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## 例

[inline-code-attrs-start title = 'get_notifications の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_notifications(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-123".to_string()),
        url_id: Some("news/article".to_string()),
        from_comment_id: Some("cmt-456".to_string()),
        viewed: Some(true),
        skip: Some(0.0),
    };
    let _response = get_notifications(configuration, params).await?;
    Ok(())
}
[inline-code-end]