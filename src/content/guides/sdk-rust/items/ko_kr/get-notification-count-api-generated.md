## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| url_id | String | No |  |
| from_comment_id | String | No |  |
| viewed | bool | No |  |

## 응답

반환: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## 예시

[inline-code-attrs-start title = 'get_notification_count 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("john.doe".to_string()),
        url_id: Some("blog/post-123".to_string()),
        from_comment_id: Some("comment789".to_string()),
        viewed: Some(true),
    };
    let _response = get_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]