---
## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|------|-------------|
| tenant_id | String | 是 |  |
| skip | f64 | 否 |  |

## 回應

回傳: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_users_response.rs)

## 範例

[inline-code-attrs-start title = 'get_tenant_users 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetTenantUsersParams = GetTenantUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetTenantUsersResponse = get_tenant_users(&configuration, params).await?;
    let _users: GetTenantUsersResponse = response;
    Ok(())
}
[inline-code-end]

---