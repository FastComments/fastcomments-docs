## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 回應

Returns: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tenant_user_response.rs)

## 範例

[inline-code-attrs-start title = 'get_tenant_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = GetTenantUserParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "user-42".into(),
    };
    let _response = get_tenant_user(&config, params).await?;
    Ok(())
}
[inline-code-end]