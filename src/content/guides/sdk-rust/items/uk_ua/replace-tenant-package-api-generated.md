## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| id | String | Так |  |
| replace_tenant_package_body | models::ReplaceTenantPackageBody | Так |  |

## Відповідь

Повертає: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'replace_tenant_package Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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