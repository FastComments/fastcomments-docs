---
## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Antwoord

Geeft terug: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesForUserResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getVotesForUser Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("/articles/2026/06/new-features");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
api->getVotesForUser(tenantId, urlId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<GetVotesForUserResponse>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetVotesForUserResponse>();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---