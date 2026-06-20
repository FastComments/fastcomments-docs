Momenteel online kijkers van een pagina: mensen wiens websocket-sessie op dit moment op de pagina geabonneerd is.
Geeft anonCount + totalCount terug (abonnees van de hele ruimte, inclusief anonieme kijkers die we niet afzonderlijk opsommen).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| after_name | String | Nee |  |
| after_user_id | String | Nee |  |

## Antwoord

Geeft terug: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_online_users Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_online_users() -> Result<PageUsersOnlineResponse, Error> {
    let params: GetOnlineUsersParams = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/article-2026".to_string(),
        after_name: Some("jane.doe".to_string()),
        after_user_id: Some("user_98765".to_string()),
    };
    let response: PageUsersOnlineResponse = get_online_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---