Trenutno prisotni gledalci strani: osebe, katerih websocket session je trenutno naročena na stran.
Vrne anonCount + totalCount (naročniki v celotni sobi, vključno z anonimnimi gledalci, ki jih ne naštevamo).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| after_name | String | Ne |  |
| after_user_id | String | Ne |  |

## Odgovor

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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