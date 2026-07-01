## Parameter

| Name     | Type   | Required | Description |
|----------|--------|----------|-------------|
| tenantId | string | Ja       |  |
| id       | string | Ja       |  |

## Antwort

Rückgabe: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCachedNotificationCountResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getCachedNotificationCount Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(utility::conversions::to_string_t("my-tenant-123"));
auto userId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user-456"));

api->getCachedNotificationCount(tenantId.value(), userId.value())
    .then([](pplx::task<std::shared_ptr<GetCachedNotificationCountResponse>> task) {
        try {
            auto response = task.get();
            // Antwort verarbeiten
        } catch (const std::exception&) {
            // Fehler behandeln
        }
    });
[inline-code-end]