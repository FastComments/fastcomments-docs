Użytkownicy strony obecnie online: osoby, których sesja websocket jest aktualnie subskrybowana na tej stronie.
Zwraca anonCount + totalCount (abonenci w całym pokoju, włącznie z anonimowymi widzami, których nie wyliczamy).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| after_name | String | Nie |  |
| after_user_id | String | Nie |  |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_online_users Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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