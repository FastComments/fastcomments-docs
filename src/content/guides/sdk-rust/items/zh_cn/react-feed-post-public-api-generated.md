## 参数

| Name | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| post_id | String | 是 |  |
| react_body_params | models::ReactBodyParams | 是 |  |
| is_undo | bool | 否 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_response.rs)

## 示例

[inline-code-attrs-start title = 'react_feed_post_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: ReactFeedPostPublicParams = ReactFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-2026-06-19".to_string(),
        react_body_params: models::ReactBodyParams {
            reaction: "like".to_string(),
            user_id: "user-9876".to_string(),
            metadata: None,
        },
        is_undo: Some(false),
        broadcast_id: Some("broadcast-42".to_string()),
        sso: Some("sso-token-abc123".to_string()),
    };
    let response: ReactFeedPostResponse = react_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---