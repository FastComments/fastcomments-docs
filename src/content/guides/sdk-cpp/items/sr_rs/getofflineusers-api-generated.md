Past komentari na stranici koji NISU trenutno online. Sortirani po displayName.  
Koristite ovo nakon što potrošite /users/online kako biste prikazali sekciju „Members“.  
Kursor paginacija po commenterName: server prolazi kroz delimični indeks {tenantId, urlId, commenterName} od afterName napred korišćenjem $gt, bez troška $skip.

## Parameters

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOfflineUsersOptions& | Yes |  |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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