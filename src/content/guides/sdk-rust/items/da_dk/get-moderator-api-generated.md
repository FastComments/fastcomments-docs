## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Returnerer: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_moderator Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "moderator-123".to_string(),
    };
    let _response: GetModeratorResponse = get_moderator(configuration, params).await?;
    Ok(())
}
[inline-code-end]