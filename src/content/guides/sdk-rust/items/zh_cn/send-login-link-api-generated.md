## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| redirect_url | String | 否 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'send_login_link 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_link_example() -> Result<(), Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9876".to_string(),
        redirect_url: Some("https://acme.example.com/welcome".to_string()),
    };
    let response: ApiEmptyResponse = send_login_link(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---