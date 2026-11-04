Eerdere commenters op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit nadat je /users/online hebt uitgeput om een "Leden"-sectie weer te geven.
Cursorpaginering op commenterName: de server loopt de partiële {tenantId, urlId, commenterName}
index vanaf afterName vooruit via $gt, zonder $skip-kosten.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | const GetOfflineUsersOptions& | Ja |  |

## Response

Geeft terug: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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