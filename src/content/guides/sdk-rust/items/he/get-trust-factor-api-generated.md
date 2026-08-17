## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | No |  |
| sso | String | No |  |

## תגובה

מחזיר: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_trust_factor_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_trust_factor'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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