## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| post_id | String | 是 |  |
| react_body_params | models::ReactBodyParams | 是 |  |
| is_undo | bool | 否 |  |
| broadcast_id | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回：[`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/react_feed_post_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'react_feed_post_public 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: ReactFeedPostPublicParams = ReactFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    post_id: "news/world/2026-election".to_string(),
    react_body_params: models::ReactBodyParams { reaction: "like".to_string() },
    is_undo: Some(false),
    broadcast_id: Some("broadcast-2026-03-25".to_string()),
    sso: Some("sso-token-6f4e2b".to_string()),
};

let response: ReactFeedPostPublic200Response = react_feed_post_public(&configuration, params).await?;
[inline-code-end]

---