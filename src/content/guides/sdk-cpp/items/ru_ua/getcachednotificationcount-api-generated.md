## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCachedNotificationCountResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getCachedNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
boost::optional<utility::string_t> preferredLocale = boost::none;
auto fallback = std::make_shared<GetCachedNotificationCountResponse>();
api->getCachedNotificationCount(tenantId, id)
.then([fallback](pplx::task<std::shared_ptr<GetCachedNotificationCountResponse>> t) {
    try {
        auto resp = t.get();
        auto result = resp ? resp : fallback;
        std::cout << "cachedNotificationCount: " << result->count << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error fetching cached notification count: " << e.what() << std::endl;
    }
});
[inline-code-end]

---