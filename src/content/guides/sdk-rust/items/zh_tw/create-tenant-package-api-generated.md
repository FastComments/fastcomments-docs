## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| create_tenant_package_body | models::CreateTenantPackageBody | 是 |  |

## 回應

回傳: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_package_response.rs)

## 範例

[inline-code-attrs-start title = 'create_tenant_package 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<CreateTenantPackageResponse, Error> {
    let create_tenant_package_body: models::CreateTenantPackageBody = models::CreateTenantPackageBody {
        name: "Premium Support".to_string(),
        plan: "enterprise".to_string(),
        seats: Some(50),
        price_cents: Some(19900),
        currency: Some("USD".to_string()),
        features: Some(vec!["priority-support".to_string(), "white-label".to_string()]),
        auto_renew: Some(true),
        notes: Some("Includes monthly account review".to_string()),
    };
    let params: CreateTenantPackageParams = CreateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_package_body,
    };
    let response: CreateTenantPackageResponse = create_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---