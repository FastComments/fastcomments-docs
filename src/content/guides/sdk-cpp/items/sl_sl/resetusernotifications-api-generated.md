## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | const ResetUserNotificationsOptions& | Da |  |

## Odgovor

Vrne: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer resetUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
ResetUserNotificationsOptions options;
options.email = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotifications(tenantId, options)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp) {
        // Obdelaj odgovor
    });
[inline-code-end]