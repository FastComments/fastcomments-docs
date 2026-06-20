## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_notification_count 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetUserNotificationCountParams = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        sso: Some("user-42.sso.example".to_owned()),
    };
    let response: GetUserNotificationCountResponse = get_user_notification_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---