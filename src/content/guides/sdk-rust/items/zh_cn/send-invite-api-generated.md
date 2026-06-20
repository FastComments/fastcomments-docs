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
async fn run_send_invite() -> Result<ApiEmptyResponse, Error> {
    let params: SendInviteParams = SendInviteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-06-19".to_string(),
        from_name: "Acme News Team".to_string(),
        subject: Some("Invitation to comment".to_string()),
        message: Some("We value your feedback on this article — join the conversation.".to_string()),
        ..Default::default()
    };

    let response: ApiEmptyResponse = send_invite(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]