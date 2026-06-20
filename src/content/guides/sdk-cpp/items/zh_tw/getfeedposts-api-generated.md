req
tenantId
afterId

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | int32_t | 否 |  |
| tags | vector<string | 否 |  |

## 回傳

回傳: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsResponse.h)

## 範例

[inline-code-attrs-start title = 'getFeedPosts 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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