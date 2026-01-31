## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | vector<string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`GetFeedPostsStats_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsStats_200_response.h)

## Example

[inline-code-attrs-start title = 'getFeedPostsStats Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
std::vector<utility::string_t> postIds = { U("post-001"), U("post-002") };
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->getFeedPostsStats(tenantId, postIds, sso)
    .then([](pplx::task<std::shared_ptr<GetFeedPostsStats_200_response>> t) {
        try {
            auto resp = t.get();
            if(!resp) resp = std::make_shared<GetFeedPostsStats_200_response>();
            (void)resp;
        } catch(const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
