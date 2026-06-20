---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| sso | String | 아니오 |  |

## 응답

반환: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_manual_badges_response.rs)

## 예제

[inline-code-attrs-start title = 'get_manual_badges 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_manual_badges() -> Result<(), Error> {
    let params: GetManualBadgesParams = GetManualBadgesParams {
        sso: Some(String::from("https://sso.acme-corp.com/authorize?tenant=acme-corp-tenant")),
    };
    let response: GetTenantManualBadgesResponse = get_manual_badges(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---