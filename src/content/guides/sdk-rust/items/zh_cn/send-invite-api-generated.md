## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| from_name | String | 是 |  |

## 响应

返回：[`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'send_invite 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = SendInviteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        from_name: "John Doe".to_string(),
        message: Some("Welcome to the platform".to_string()),
        ..Default::default()
    };
    let _ = send_invite(configuration, params).await?;
    Ok(())
}
[inline-code-end]