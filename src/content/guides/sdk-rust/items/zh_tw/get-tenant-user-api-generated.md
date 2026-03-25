## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_200_response.rs)

## 範例

[inline-code-attrs-start title = 'get_tenant_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetTenantUserParams = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-9a4f2e".to_string(),
        expand: Some(vec!["roles".to_string(), "preferences".to_string()]),
    };
    let user_response: GetTenantUser200Response = get_tenant_user(&configuration, params).await?;
    println!("{:#?}", user_response);
    Ok(())
}
[inline-code-end]

---