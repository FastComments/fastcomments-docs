---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_tenant_body | models::CreateTenantBody | 是 |  |

## 回應

回傳: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_response.rs)

## 範例

[inline-code-attrs-start title = 'create_tenant 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn create_tenant_example() -> Result<CreateTenantResponse, Error> {
    let params: CreateTenantParams = CreateTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_body: models::CreateTenantBody {
            name: "Acme Corporation".to_string(),
            primary_domain: "acme-corp.com".to_string(),
            contact_email: "ops@acme-corp.com".to_string(),
            api_domain_configuration: Some(ApiDomainConfiguration {
                enabled: true,
                domain: "comments.acme-corp.com".to_string(),
            }),
            billing_info: Some(BillingInfo {
                plan: "pro".to_string(),
                billing_contact_email: "billing@acme-corp.com".to_string(),
            }),
            imported_sites: Some(vec![ImportedSiteType {
                site_type: "news/article".to_string(),
                site_id: "acme-news".to_string(),
            }]),
        },
    };
    let response: CreateTenantResponse = create_tenant(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---