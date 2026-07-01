Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Поточні онлайн глядачі сторінки: люди, чиї WebSocket‑сесії підписані на сторінку в даний момент.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).
Повертає anonCount + totalCount (підписники по всій кімнаті, включаючи анонімних глядачів, яких ми не перелічуємо).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOnlineUsersOptions& | Yes |  |

## Response

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
auto options = std::make_shared<GetOnlineUsersOptions>();
options->maxResults = boost::optional<int>(100);
options->includeInactive = boost::optional<bool>(false);
api->getOnlineUsers(tenantId, urlId, *options).then([](pplx::task<std::shared_ptr<PageUsersOnlineResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]