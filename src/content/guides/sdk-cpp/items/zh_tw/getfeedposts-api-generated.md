## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| afterId | string | 否 |  |
| limit | int32_t | 否 |  |
| tags | vector<string | 否 |  |

## 回應

回傳: [`GetFeedPosts_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPosts_200_response.h)

## 範例

[inline-code-attrs-start title = 'getFeedPosts 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId(utility::string_t(U("post_456")));
boost::optional<int32_t> limit(50);
std::vector<utility::string_t> tagVec{U("news"), U("announcement")};
boost::optional<std::vector<utility::string_t>> tags(tagVec);
api->getFeedPosts(tenantId, afterId, limit, tags)
.then([](std::shared_ptr<GetFeedPosts_200_response> resp) {
    auto copy = std::make_shared<GetFeedPosts_200_response>(*resp);
    (void)copy;
});
[inline-code-end]

---