## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| url_id | String | 否 |  |
| from_comment_id | String | 否 |  |
| viewed | bool | 否 |  |
| skip | f64 | 否 |  |

## 回應

回傳：[`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## 範例

[inline-code-attrs-start title = 'get_notifications 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_notifications() -> Result<(), Error> {
    let params: GetNotificationsParams = GetNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-9a7b".to_string()),
        url_id: Some("news/article/launch-announcement".to_string()),
        from_comment_id: Some("cmt-1024".to_string()),
        viewed: Some(false),
        skip: Some(0.0),
    };
    let notifications: GetNotificationsResponse = get_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---