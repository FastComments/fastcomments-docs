## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| meta | String | No |  |
| skip | f64 | No |  |

## 回應

回傳: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_tenants 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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