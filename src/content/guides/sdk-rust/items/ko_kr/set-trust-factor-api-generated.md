---
## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| user_id | String | 아니요 |  |
| trust_factor | String | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/set_user_trust_factor_response.rs)

## 예제

[inline-code-attrs-start title = 'set_trust_factor 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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