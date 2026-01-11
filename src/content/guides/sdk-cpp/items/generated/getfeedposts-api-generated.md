## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| limit | int32_t | No |  |
| tags | vector<string | No |  |

## Response

Returns: [`GetFeedPosts_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPosts_200_response.h)

## Example

[inline-code-attrs-start title = 'getFeedPosts Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId("my-tenant-123");
boost::optional<utility::string_t> afterId(utility::string_t("post_98765"));
boost::optional<int32_t> limit(25);
std::vector<utility::string_t> tagVec{ utility::string_t("news"), utility::string_t("featured") };
boost::optional<std::vector<utility::string_t>> tags(tagVec);
api->getFeedPosts(tenantId, afterId, limit, tags).then([](pplx::task<std::shared_ptr<GetFeedPosts_200_response>> t) {
    try {
        auto resp = t.get();
        auto respCopy = std::make_shared<GetFeedPosts_200_response>(*resp);
        (void)respCopy;
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]
