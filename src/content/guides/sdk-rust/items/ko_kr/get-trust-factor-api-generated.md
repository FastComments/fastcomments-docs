## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| user_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

Returns: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_trust_factor_response.rs)

## 예시

[inline-code-attrs-start title = 'get_trust_factor 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetTrustFactorParams {
        tenant_id: "acme-corp-tenant".into(),
        user_id: Some("user-12345".into()),
        sso: Some("sso-provider".into()),
    };
    let _response = get_trust_factor(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---