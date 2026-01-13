## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| skip | f64 | 否 |  |

## 回應

回傳: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_tenant_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_tenant_users() -> Result<GetTenantUsers200Response, Error> {
    let params: GetTenantUsersParams = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let response: GetTenantUsers200Response = get_tenant_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---