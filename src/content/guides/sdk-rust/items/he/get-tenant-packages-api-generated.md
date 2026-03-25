---
## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| skip | f64 | לא |  |

## תגובה

מחזיר: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_packages_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'get_tenant_packages דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantPackagesParams = GetTenantPackagesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let packages: GetTenantPackages200Response = get_tenant_packages(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---