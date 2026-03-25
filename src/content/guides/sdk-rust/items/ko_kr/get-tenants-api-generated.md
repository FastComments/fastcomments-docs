## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| meta | String | 아니오 |  |
| skip | f64 | 아니오 |  |

## 응답

반환: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenants_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_tenants 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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