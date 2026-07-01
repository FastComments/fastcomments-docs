Momenteel online kijkers van een pagina: mensen wiens websocket‑sessie op dit moment op de pagina geabonneerd is.  
Retourneert anonCount + totalCount (abonnees in de hele ruimte, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| after_name | String | Nee |  |
| after_user_id | String | Nee |  |

## Response

Retourneert: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_online_users Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]