## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Нет |  |
| urlId | string | Нет |  |
| fromCommentId | string | Нет |  |
| viewed | bool | Нет |  |
| type | string | Нет |  |

## Ответ

Возвращает: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("article-456");
boost::optional<utility::string_t> fromCommentId = U("cmt-789");
boost::optional<bool> viewed = true;
boost::optional<utility::string_t> type = U("mention");

auto task = api->getNotificationCount(tenantId, userId, urlId, fromCommentId, viewed, type)
.then([](pplx::task<std::shared_ptr<GetNotificationCountResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) return std::make_shared<GetNotificationCountResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetNotificationCountResponse>();
    }
});
[inline-code-end]

---