## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| meta | String | לא |  |
| skip | f64 | לא |  |

## תגובה

מחזיר: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_tenants'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantsParams = GetTenantsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        meta: Some("news/article".to_string()),
        skip: Some(10.0),
    };
    let response: GetTenants200Response = get_tenants(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]

---