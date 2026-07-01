## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | const ResetUserNotificationsOptions& | Da |  |

## Odgovor

Vraća: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Primjer

[inline-code-attrs-start title = 'resetUserNotifications Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
ResetUserNotificationsOptions options;
options.email = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotifications(tenantId, options)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp) {
        // Obradi odgovor
    });
[inline-code-end]