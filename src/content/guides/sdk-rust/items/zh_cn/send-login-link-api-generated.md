## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| redirect_url | String | 否 |  |

## 响应

返回：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'send_login_link 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn send_login_example() -> Result<FlagCommentPublic200Response, Error> {
    let params: SendLoginLinkParams = SendLoginLinkParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("user-98765"),
        redirect_url: Some(String::from("https://acme.example.com/dashboard")),
    };
    let response: FlagCommentPublic200Response = send_login_link(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---