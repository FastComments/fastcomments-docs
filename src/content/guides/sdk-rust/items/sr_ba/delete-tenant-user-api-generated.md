## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| id | String | Da |  |
| delete_comments | String | Ne |  |
| comment_delete_mode | String | Ne |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'delete_tenant_user Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteTenantUserParams = DeleteTenantUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "user-8421".to_string(),
        delete_comments: Some("yes".to_string()),
        comment_delete_mode: Some("permanent".to_string()),
    };
    let _response: ApiEmptyResponse = delete_tenant_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---