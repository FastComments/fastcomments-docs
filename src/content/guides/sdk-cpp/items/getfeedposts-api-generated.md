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
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId(U("post_456"));
boost::optional<int32_t> limit(20);
boost::optional<std::vector<utility::string_t>> tags(std::vector<utility::string_t>{U("news"), U("announcement")});

api->getFeedPosts(tenantId, afterId, limit, tags)
.then([](std::shared_ptr<GetFeedPosts_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<GetFeedPosts_200_response>(*resp);
    }
})
.wait();
[inline-code-end]
