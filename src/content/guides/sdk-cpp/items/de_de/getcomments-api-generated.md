## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Yes |  |
| options | const GetCommentsOptions& | Yes |  |

## Antwort

Rückgabe: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCommentsOptions options;
options.page = 1;
options.limit = 50;
options.sort = U("newest");
options.authorEmail = boost::optional<utility::string_t>(U("user@example.com"));
api->getComments(tenantId, options).then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> task) {
    try {
        auto response = task.get();
        // Antwort nach Bedarf verwenden
    } catch (const std::exception& e) {
        // Fehler behandeln
    }
});
[inline-code-end]