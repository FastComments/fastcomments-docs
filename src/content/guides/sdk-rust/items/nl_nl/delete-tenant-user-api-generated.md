## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| delete_comments | String | Nee |  |
| comment_delete_mode | String | Nee |  |

## Respons

Returns: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'delete_tenant_user Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteTenantUserParams {
        tenant_id: "acme-corp".into(),
        id: "user-123".into(),
        delete_comments: Some("true".into()),
        comment_delete_mode: Some("hard".into()),
    };
    delete_tenant_user(&config, params).await?;
    Ok(())
}
[inline-code-end]