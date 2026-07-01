## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| options | const GetVotesForUserOptions& | Sì |  |

## Risposta

Restituisce: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesForUserResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio getVotesForUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("post-456"));
GetVotesForUserOptions options;
options.page = boost::optional<int>(2);
options.pageSize = boost::optional<int>(50);
api->getVotesForUser(tenantId, urlId, options).then([](std::shared_ptr<GetVotesForUserResponse> response) {
    if (response) {
        // gestire la risposta, ad esempio, iterare i voti
    }
});
[inline-code-end]