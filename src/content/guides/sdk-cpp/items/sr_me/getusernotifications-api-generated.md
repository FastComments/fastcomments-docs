## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Не |  |
| pageSize | int32_t | Не |  |
| afterId | string | Не |  |
| includeContext | bool | Не |  |
| afterCreatedAt | int64_t | Не |  |
| unreadOnly | bool | Не |  |
| dmOnly | bool | Не |  |
| noDm | bool | Не |  |
| includeTranslations | bool | Не |  |
| includeTenantNotifications | bool | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Пример

[inline-code-attrs-start title = 'getUserNotifications Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
api->getUserNotifications(
    tenantId,
    boost::optional<utility::string_t>(U("post-456")),
    boost::optional<int32_t>(50),
    boost::optional<utility::string_t>(U("notif-789")),
    boost::optional<bool>(true),
    boost::optional<int64_t>(1625097600000LL),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<bool>(false),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetMyNotificationsResponse>();
        // користите resp, нпр. прегледајте поља
    } catch(const std::exception &e) {
    }
});
[inline-code-end]