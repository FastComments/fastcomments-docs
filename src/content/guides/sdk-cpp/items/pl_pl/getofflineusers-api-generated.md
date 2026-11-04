Osoby komentujące wcześniej na stronie, które NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members".
Paginacja kursorowa po commenterName: serwer przechodzi po częściowym indeksie {tenantId, urlId, commenterName}
od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| options | const GetOfflineUsersOptions& | Tak |  |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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