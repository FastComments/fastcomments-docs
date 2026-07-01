## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| delete_comments | String | No |  |
| comment_delete_mode | String | No |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_tenant_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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