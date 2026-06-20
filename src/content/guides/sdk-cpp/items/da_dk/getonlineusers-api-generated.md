---
Aktuelt online seere af en side: personer, hvis websocket-session er tilmeldt siden lige nu.
Returnerer anonCount + totalCount (abonnenter på hele rummet, inklusive anonyme seere, som vi ikke opregner).

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nej |  |
| afterUserId | string | Nej |  |

## Svar

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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