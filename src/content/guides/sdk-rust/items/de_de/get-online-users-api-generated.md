Aktuell online betrachtende Besucher einer Seite: Personen, deren Websocket‑Sitzung derzeit für die Seite abonniert ist. Gibt anonCount + totalCount zurück (räumweite Abonnenten, einschließlich anonymer Betrachter, die wir nicht aufzählen).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| after_name | String | Nein |  |
| after_user_id | String | Nein |  |

## Antwort

Rückgabe: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_online_users Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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