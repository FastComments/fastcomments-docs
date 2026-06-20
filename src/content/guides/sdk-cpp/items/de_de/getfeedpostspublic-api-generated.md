req
tenantId
afterId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nein |  |
| limit | int32_t | Nein |  |
| tags | vector<string | Nein |  |
| sso | string | Nein |  |
| isCrawler | bool | Nein |  |
| includeUserInfo | bool | Nein |  |

## Antwort

Gibt zurück: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getFeedPostsPublic Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("post_456"));
boost::optional<int32_t> limit = boost::optional<int32_t>(25);
std::vector<utility::string_t> tagList = { U("news"), U("announcement") };
boost::optional<std::vector<utility::string_t>> tags = boost::optional<std::vector<utility::string_t>>(tagList);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<bool> isCrawler = boost::optional<bool>(false);
boost::optional<bool> includeUserInfo = boost::optional<bool>(true);

api->getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo)
    .then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> t){
        try {
            auto resp = t.get();
            if(!resp) resp = std::make_shared<PublicFeedPostsResponse>();
        } catch (const std::exception&) {
            auto fallback = std::make_shared<PublicFeedPostsResponse>();
        }
    });
[inline-code-end]

---