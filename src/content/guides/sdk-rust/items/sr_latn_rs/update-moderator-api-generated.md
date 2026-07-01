## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_moderator_body | models::UpdateModeratorBody | Yes |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'update_moderator Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = UpdateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-42".to_string(),
        update_moderator_body: models::UpdateModeratorBody {
            name: Some("Alice Smith".to_string()),
            email: Some("alice.smith@example.com".to_string()),
            is_active: Some(true),
        },
    };
    update_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]