Frühere Kommentatoren auf der Seite, die derzeit NICHT online sind. Nach displayName sortiert.
Verwenden Sie dies, nachdem /users/online erschöpft wurde, um einen "Mitglieder"-Abschnitt darzustellen.
Cursor-Paginierung auf commenterName: der Server durchläuft das partielle {tenantId, urlId, commenterName}
index von afterName vorwärts via $gt, keine $skip-Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| after_name | String | Nein |  |
| after_user_id | String | Nein |  |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline_users() -> Result<PageUsersOfflineResponse, Error> {
    let params: GetOfflineUsersParams = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/world/today".to_string(),
        after_name: Some("jane.smith".to_string()),
        after_user_id: Some("user-1024".to_string()),
    };
    let response: PageUsersOfflineResponse = get_offline_users(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---