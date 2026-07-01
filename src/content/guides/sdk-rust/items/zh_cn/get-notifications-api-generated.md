## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| url_id | String | 否 |  |
| from_comment_id | String | 否 |  |
| viewed | bool | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notifications_response.rs)

## 示例

[inline-code-attrs-start title = 'get_notifications 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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