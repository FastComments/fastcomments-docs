## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odziv

Vrne: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCachedNotificationCountResponse.h)

## Primer

[inline-code-attrs-start title = 'getCachedNotificationCount Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(utility::conversions::to_string_t("my-tenant-123"));
auto userId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user-456"));

api->getCachedNotificationCount(tenantId.value(), userId.value())
    .then([](pplx::task<std::shared_ptr<GetCachedNotificationCountResponse>> task) {
        try {
            auto response = task.get();
            // obdelaj odgovor
        } catch (const std::exception&) {
            // obravnava napake
        }
    });
[inline-code-end]