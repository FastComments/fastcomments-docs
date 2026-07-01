## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | Yes |  |
| update_comments | String | No |  |

## Yanıt

Döndürür: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Örnek

[inline-code-attrs-start title = 'replace_tenant_user Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ReplaceTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-12345".to_string(),
        replace_tenant_user_body: ReplaceTenantUserBody::default(),
        update_comments: Some("Update user role".to_string()),
    };
    replace_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]