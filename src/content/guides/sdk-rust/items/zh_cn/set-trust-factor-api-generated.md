## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| user_id | String | 否 |  |
| trust_factor | String | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## 示例

[inline-code-attrs-start title = 'set_trust_factor 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_trust() -> Result<(), Error> {
    let params = SetTrustFactorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-123".to_string()),
        trust_factor: Some("high".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = set_trust_factor(&configuration, params).await?;
    Ok(())
}
[inline-code-end]