## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Returnerer: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_moderator Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_moderator() -> Result<GetModeratorResponse, Error> {
    let params: GetModeratorParams = GetModeratorParams {
        tenant_id: "acme-newsroom".to_string(),
        id: "mod-jane-smith-001".to_string(),
    };
    let include_permissions: Option<bool> = Some(true);
    let moderator: GetModeratorResponse = get_moderator(&configuration, params).await?;
    Ok(moderator)
}
[inline-code-end]

---