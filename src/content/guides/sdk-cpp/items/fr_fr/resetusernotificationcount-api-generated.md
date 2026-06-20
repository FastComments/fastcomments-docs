## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotificationCount(tenantId, sso)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp){
        if(!resp) resp = std::make_shared<ResetUserNotificationsResponse>();
    })
    .wait();
[inline-code-end]

---