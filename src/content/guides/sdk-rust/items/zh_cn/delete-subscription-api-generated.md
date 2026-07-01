## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| user_id | String | No |  |

## 响应

返回: [`DeleteSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_subscription_api_response.rs)

## 示例

[inline-code-attrs-start title = 'delete_subscription 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteSubscriptionParams {
        tenant_id: "acme-corp".to_string(),
        id: "sub-2024-09".to_string(),
        user_id: Some("user-42".to_string()),
    };
    let _response = delete_subscription(&config, params).await?;
    Ok(())
}
[inline-code-end]