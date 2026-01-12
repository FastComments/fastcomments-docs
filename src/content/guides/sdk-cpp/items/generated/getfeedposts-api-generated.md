
req
tenantId
afterId

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
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(utility::string_t(U("post_456")));
boost::optional<int32_t> limit = boost::optional<int32_t>(20);
std::vector<utility::string_t> tagList = { utility::string_t(U("news")), utility::string_t(U("product-updates")) };
boost::optional<std::vector<utility::string_t>> tags = boost::optional<std::vector<utility::string_t>>(tagList);
api->getFeedPosts(tenantId, afterId, limit, tags).then([](pplx::task<std::shared_ptr<GetFeedPosts_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto localCopy = std::make_shared<GetFeedPosts_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
