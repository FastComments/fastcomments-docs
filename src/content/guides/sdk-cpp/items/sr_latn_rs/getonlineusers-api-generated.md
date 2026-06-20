Trenutno-prisotni posetioci stranice: ljudi čija je websocket sesija pretplaćena na stranicu upravo sada.
Vraća anonCount + totalCount (pretplatnike u okviru sobe, uključujući anonimne posetioce koje ne navodimo).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Primer

[inline-code-attrs-start title = 'getOnlineUsers Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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