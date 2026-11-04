Tidligere kommentatorer på siden, som IKKE er online i øjeblikket. Sorteret efter displayName.
Brug denne, efter du har opbrugt /users/online, for at rendere et "Members"-afsnit.
Cursor-paginering på commenterName: serveren gennemløber det partielle {tenantId, urlId, commenterName}
indeks frem fra afterName via $gt, uden $skip-omkostninger.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | const GetOfflineUsersOptions& | Ja |  |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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