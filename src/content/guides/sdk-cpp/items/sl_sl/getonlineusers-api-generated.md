Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Trenutno prisotni gledalci strani: ljudje, katerih sejo WebSocket je v tem trenutku naročena na stran.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).
Vrne anonCount + totalCount (naročniki po celotni sobi, vključno z anonimnimi gledalci, ki jih ne naštejemo).

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| options | const GetOnlineUsersOptions& | Da |  |

## Response

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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