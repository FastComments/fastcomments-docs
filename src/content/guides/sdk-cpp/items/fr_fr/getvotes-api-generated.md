## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |

## Réponse

Renvoie : [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getVotes'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<int> limit = 50;
auto fallback = std::make_shared<GetVotesResponse>();
api->getVotes(utility::conversions::to_string_t("my-tenant-123"), utility::conversions::to_string_t("article-9876"))
.then([fallback, limit](pplx::task<std::shared_ptr<GetVotesResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = fallback;
        if (limit) {
            auto processed = std::make_shared<GetVotesResponse>(*resp);
        }
    } catch (const std::exception& e) {
        auto errorResp = std::make_shared<GetVotesResponse>();
    }
});
[inline-code-end]

---