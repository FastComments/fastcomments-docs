## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| meta | String | לא |  |
| skip | f64 | לא |  |

## תשובה

מחזיר: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_tenants'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: String::from("acme-corp-tenant"),
        meta: Some(String::from("include=domains,settings")),
        skip: Some(10.0),
    };
    let response: GetTenants200Response = get_tenants(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---