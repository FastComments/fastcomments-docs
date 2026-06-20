Текущие онлайн-зрители страницы — это люди, чьи WebSocket-сессии в данный момент подписаны на эту страницу.
Возвращает anonCount + totalCount (подписчики по всей комнате, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Нет |  |
| afterUserId | string | Нет |  |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("https://www.example.com/posts/2026/06/19/introduction");
boost::optional<utility::string_t> afterName = boost::optional<utility::string_t>(U("alice@example.com"));
boost::optional<utility::string_t> afterUserId;

api->getOnlineUsers(tenantId, urlId, afterName, afterUserId)
.then([](pplx::task<std::shared_ptr<PageUsersOnlineResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<PageUsersOnlineResponse>();
        return resp;
    } catch(...) {
        return std::make_shared<PageUsersOnlineResponse>();
    }
}).wait();
[inline-code-end]

---