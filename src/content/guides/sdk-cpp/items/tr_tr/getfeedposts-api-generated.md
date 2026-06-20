req
tenantId
afterId

## Parametreler

| Adı | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| afterId | string | Hayır |  |
| limit | int32_t | Hayır |  |
| tags | vector<string | Hayır |  |

## Yanıt

Döndürür: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsResponse.h)

## Örnek

[inline-code-attrs-start title = 'getFeedPosts Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("post_987"));
boost::optional<int32_t> limit = boost::optional<int32_t>(50);
boost::optional<std::vector<utility::string_t>> tags = boost::optional<std::vector<utility::string_t>>(std::vector<utility::string_t>{ U("release"), U("security") });
api->getFeedPosts(tenantId, afterId, limit, tags)
    .then([](pplx::task<std::shared_ptr<GetFeedPostsResponse>> t) {
        try {
            auto resp = t.get();
            auto result = resp ? resp : std::make_shared<GetFeedPostsResponse>();
            std::cout << "Received feed response" << std::endl;
        } catch (const std::exception& ex) {
            std::cerr << "getFeedPosts failed: " << ex.what() << std::endl;
        }
    });
[inline-code-end]

---