## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| postIds | vector<string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getFeedPostsStats'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
std::vector<utility::string_t> postIds = {
    utility::conversions::to_string_t("post-001"),
    utility::conversions::to_string_t("post-002")
};
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");

api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](std::shared_ptr<FeedPostsStatsResponse> response) {
        (void)response;
    });
[inline-code-end]