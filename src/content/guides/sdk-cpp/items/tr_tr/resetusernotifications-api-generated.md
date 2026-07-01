---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const ResetUserNotificationsOptions& | Yes |  |

## Yanıt

Döndürür: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## Örnek

[inline-code-attrs-start title = 'resetUserNotifications Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
ResetUserNotificationsOptions options;
options.email = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotifications(tenantId, options)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp) {
        // Yanıtı işle
    });
[inline-code-end]

---