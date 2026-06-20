## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 示例

[inline-code-attrs-start title = 'lock_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_lock_comment() -> Result<ApiEmptyResponse, Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        comment_id: "cmt-20240618-42".to_owned(),
        broadcast_id: "news/article/2024-06-18".to_owned(),
        sso: Some("user-12345-sso-token".to_owned()),
    };
    let response: ApiEmptyResponse = lock_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---