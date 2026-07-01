Enable oder deaktivieren Sie Benachrichtigungen für eine Seite. Wenn Benutzer eine Seite abonniert haben, werden Benachrichtigungen für neue Stammkommentare erstellt und auch  

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| url | string | Ja |  |
| pageTitle | string | Ja |  |
| subscribedOrUnsubscribed | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Beispiel

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://example.com/articles/awesome-article");
utility::string_t pageTitle = U("Awesome Article");
utility::string_t subscription = U("subscribed");
boost::optional<utility::string_t> sso = boost::make_optional<utility::string_t>(U("sso-token-789"));

api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscription, sso)
    .then([](std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse> resp) {
        // process resp if needed
    });
[inline-code-end]