## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Odpowiedź

Zwraca: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderator_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_moderator'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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