req
tenantId
afterId

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| afterId | string | 아니오 |  |
| limit | int32_t | 아니오 |  |
| tags | vector<string | 아니오 |  |
| sso | string | 아니오 |  |
| isCrawler | bool | 아니오 |  |
| includeUserInfo | bool | 아니오 |  |

## 응답

반환: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## 예제

[inline-code-attrs-start title = 'getFeedPostsPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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