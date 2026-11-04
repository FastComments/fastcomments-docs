Prošli komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online za prikaz sekcije "Members".
Cursor paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName}
indeks od afterName naprijed pomoću $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| options | const GetOfflineUsersOptions& | Da |  |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer za getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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