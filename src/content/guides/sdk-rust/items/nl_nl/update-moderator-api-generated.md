## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| update_moderator_body | models::UpdateModeratorBody | Ja |  |

## Respons

Retourneert: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'update_moderator Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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