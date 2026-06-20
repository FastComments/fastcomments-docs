---
Trenutno povezani gledalci strani: osebe, katerih websocket seja je trenutno naročena na stran.
Vrača anonCount + totalCount (naročniki na ravni sobe, vključno z anonimnimi gledalci, ki jih ne naštevamo).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vrača: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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