## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Ні |  |
| pageSize | int32_t | Ні |  |
| afterId | string | Ні |  |
| includeContext | bool | Ні |  |
| afterCreatedAt | int64_t | Ні |  |
| unreadOnly | bool | Ні |  |
| dmOnly | bool | Ні |  |
| noDm | bool | Ні |  |
| includeTranslations | bool | Ні |  |
| includeTenantNotifications | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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
        // використовуйте resp, наприклад, перевірте поля
    } catch(const std::exception &e) {
    }
});
[inline-code-end]