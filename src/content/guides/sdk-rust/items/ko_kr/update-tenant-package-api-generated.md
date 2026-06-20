## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| id | String | 예 |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | 예 |  |

## 응답

반환: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 예제

[inline-code-attrs-start title = 'update_tenant_package 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update_package() -> Result<(), Error> {
    let params: UpdateTenantPackageParams = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "package-premium".to_string(),
        update_tenant_package_body: models::UpdateTenantPackageBody {
            name: Some("Premium".to_string()),
            description: Some("Premium moderation and analytics package".to_string()),
            price_cents: Some(2999),
            features: Some(vec![
                "moderation".to_string(),
                "analytics".to_string(),
                "priority-support".to_string(),
            ]),
            active: Some(true),
        },
    };

    let _response: ApiEmptyResponse = update_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---