## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回: [`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## 示例

[inline-code-attrs-start title = 'pin_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: PinCommentParams = PinCommentParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "cmt-987654321".to_string(),
    broadcast_id: "news/article/2026-03-25".to_string(),
    sso: Some("user-12345-ssotoken".to_string()),
};
let response: PinComment200Response = pin_comment(&configuration, params).await?;
[inline-code-end]

---