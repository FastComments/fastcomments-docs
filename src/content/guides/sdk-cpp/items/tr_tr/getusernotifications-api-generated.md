## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| pageSize | int32_t | Hayır |  |
| afterId | string | Hayır |  |
| includeContext | bool | Hayır |  |
| afterCreatedAt | int64_t | Hayır |  |
| unreadOnly | bool | Hayır |  |
| dmOnly | bool | Hayır |  |
| noDm | bool | Hayır |  |
| includeTranslations | bool | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotifications_200_response.h)

## Örnek

[inline-code-attrs-start title = 'getUserNotifications Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<int32_t> pageSize = 50;
boost::optional<utility::string_t> afterId = utility::conversions::to_string_t("notif_98765");
boost::optional<bool> includeContext = true;
boost::optional<int64_t> afterCreatedAt = static_cast<int64_t>(1672531200);
boost::optional<bool> unreadOnly = true;
boost::optional<bool> dmOnly = false;
boost::optional<bool> noDm = false;
boost::optional<bool> includeTranslations = true;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");

api->getUserNotifications(tenantId, pageSize, afterId, includeContext, afterCreatedAt, unreadOnly, dmOnly, noDm, includeTranslations, sso)
.then([](pplx::task<std::shared_ptr<GetUserNotifications_200_response>> task){
    try {
        auto resp = task.get();
        auto copy = std::make_shared<GetUserNotifications_200_response>(*resp);
        return copy;
    } catch (...) {
        return std::shared_ptr<GetUserNotifications_200_response>();
    }
});
[inline-code-end]

---