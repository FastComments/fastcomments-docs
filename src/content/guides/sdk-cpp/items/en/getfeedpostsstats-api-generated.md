## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | vector<string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FeedPostsStatsResponse.h)

## Example

[inline-code-attrs-start title = 'getFeedPostsStats Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIds = { U("post-1001"), U("post-1002"), U("post-1003") };
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<FeedPostsStatsResponse>> previous) {
        try {
            auto stats = previous.get();
            if (!stats) stats = std::make_shared<FeedPostsStatsResponse>();
            // process stats here (e.g., inspect fields, update UI)
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
