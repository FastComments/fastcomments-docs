## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| replace_tenant_package_body | models::ReplaceTenantPackageBody | 예 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'replace_tenant_package 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_replace_package() -> Result<FlagCommentPublic200Response, Error> {
    let params: ReplaceTenantPackageParams = ReplaceTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "enterprise-package-2026".to_string(),
        replace_tenant_package_body: models::ReplaceTenantPackageBody {
            name: "Acme Enterprise".to_string(),
            plan: "enterprise".to_string(),
            seats: Some(50),
            allowed_domains: Some(vec![
                "acme.com".to_string(),
                "news.acme.com".to_string(),
            ]),
        },
    };

    let response: FlagCommentPublic200Response = replace_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---