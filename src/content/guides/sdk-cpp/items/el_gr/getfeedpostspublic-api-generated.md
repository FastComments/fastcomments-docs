req
tenantId
afterId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| limit | int32_t | Όχι |  |
| tags | vector<string | Όχι |  |
| sso | string | Όχι |  |
| isCrawler | bool | Όχι |  |
| includeUserInfo | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getFeedPostsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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