## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| broadcast_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回：[`PinComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/pin_comment_200_response.rs)

## 示例

[inline-code-attrs-start title = 'un_pin_comment 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_unpin() -> Result<(), Error> {
    let params: UnPinCommentParams = UnPinCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-20260325-7a9".to_string(),
        broadcast_id: "news/homepage/launch-article".to_string(),
        sso: Some("sso-jwt-user-0a1b2c3d".to_string()),
    };
    let response: PinComment200Response = un_pin_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---