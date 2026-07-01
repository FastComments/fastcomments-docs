Obecnie online oglądający stronę: osoby, których sesja websocket jest aktualnie subskrybowana do tej strony.  
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, w tym anonimowi oglądający, których nie wymieniamy).

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| after_name | String | Nie |  |
| after_user_id | String | Nie |  |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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