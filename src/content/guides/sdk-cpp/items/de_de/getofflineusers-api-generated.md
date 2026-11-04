Frühere Kommentatoren auf der Seite, die NICHT aktuell online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem /users/online erschöpft ist, um einen „Members“-Abschnitt darzustellen.
Cursor-Paginierung über commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts via $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | const GetOfflineUsersOptions& | Ja |  |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]