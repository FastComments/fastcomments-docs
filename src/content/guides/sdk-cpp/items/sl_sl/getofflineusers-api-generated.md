Prejšnji komentatorji na strani, KI NI TRENUTNO online. Razvrščeni po displayName.  
Uporabite to po izčrpanju /users/online za izris odseka "Members".  
Cursor paginiranje po commenterName: strežnik hodi po delnem {tenantId, urlId, commenterName} indeksu od afterName naprej preko $gt, brez stroška $skip.

## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetOfflineUsersOptions& | Yes |  |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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