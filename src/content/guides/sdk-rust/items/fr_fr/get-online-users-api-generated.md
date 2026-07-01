---
Visionneurs actuellement en ligne d'une page : les personnes dont la session websocket est abonnée à la page en ce moment.  
Renvoie anonCount + totalCount (abonnés de la salle, y compris les visionneurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| after_name | String | Non |  |
| after_user_id | String | Non |  |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_online_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---