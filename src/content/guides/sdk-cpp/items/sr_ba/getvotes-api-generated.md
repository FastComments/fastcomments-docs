## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |

## Odgovor

Vraća: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## Primer

[inline-code-attrs-start title = 'getVotes Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> extraHeader = boost::none;

api->getVotes(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetVotesResponse>> task) {
    try {
        auto original = task.get();
        auto response = std::make_shared<GetVotesResponse>(*original);
    } catch (...) {
        // obrada greške
    }
});
[inline-code-end]