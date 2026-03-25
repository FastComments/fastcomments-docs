## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| is_flagged | bool | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'flag_comment_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_flag() -> Result<FlagCommentPublic200Response, Error> {
    let params: FlagCommentPublicParams = FlagCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2026-03-25-8a7b6c".to_string(),
        is_flagged: true,
        sso: Some("sso-token-user-123".to_string()),
    };
    let response: FlagCommentPublic200Response = flag_comment_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---