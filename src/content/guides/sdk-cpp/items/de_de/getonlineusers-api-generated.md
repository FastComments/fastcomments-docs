Derzeit online betrachtende Besucher einer Seite: Personen, deren Websocket‑Sitzung gerade die Seite abonniert.  
Gibt `anonCount + totalCount` zurück (räumweite Abonnenten, einschließlich anonymer Viewer, die wir nicht auflisten).

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | const GetOnlineUsersOptions& | Ja |  |

## Response

Rückgabe: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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