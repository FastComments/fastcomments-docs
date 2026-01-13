## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| pageSize | int32_t | Нет |  |
| afterId | string | Нет |  |
| includeContext | bool | Нет |  |
| afterCreatedAt | int64_t | Нет |  |
| unreadOnly | bool | Нет |  |
| dmOnly | bool | Нет |  |
| noDm | bool | Нет |  |
| includeTranslations | bool | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotifications_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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