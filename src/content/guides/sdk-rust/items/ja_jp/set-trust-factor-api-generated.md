---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| user_id | String | 任意 |  |
| trust_factor | String | 任意 |  |
| sso | String | 任意 |  |

## レスポンス

返却値: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## 例

[inline-code-attrs-start title = 'set_trust_factor の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_user_trust() -> Result<SetUserTrustFactorResponse, Error> {
    let params: SetTrustFactorParams = SetTrustFactorParams {
        user_id: Some("user-9821".to_string()),
        trust_factor: Some("high".to_string()),
        sso: Some("okta-acme-corp".to_string()),
    };

    let response: SetUserTrustFactorResponse = set_trust_factor(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---