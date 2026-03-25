---
Aktivieren oder Deaktivieren von Benachrichtigungen für eine Seite. Wenn Benutzer für eine Seite abonniert sind, werden Benachrichtigungen für neue Root-Kommentare erstellt, und auch

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| url | String | Ja |  |
| page_title | String | Ja |  |
| subscribed_or_unsubscribed | String | Ja |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für update_user_notification_page_subscription_status'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<UpdateUserNotificationStatus200Response, Error> {
    let params = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("article-12345"),
        url: String::from("https://news.acme.com/articles/2026/03/25/advances-in-ai"),
        page_title: String::from("Advances in AI: What to Expect in 2026"),
        subscribed_or_unsubscribed: String::from("subscribed"),
        sso: Some(String::from("user-jwt-xyz123")),
    };
    let response = update_user_notification_page_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---