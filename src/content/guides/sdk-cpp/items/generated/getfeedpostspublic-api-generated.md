## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| limit | int32_t | No |  |
| tags | vector<string | No |  |
| sso | string | No |  |
| isCrawler | bool | No |  |
| includeUserInfo | bool | No |  |

## Response

Returns: [`GetFeedPostsPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'getFeedPostsPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId{ utility::string_t(U("post_abcdef")) };
boost::optional<int32_t> limit{ 20 };
boost::optional<std::vector<utility::string_t>> tags{ std::vector<utility::string_t>{ U("news"), U("announcement") } };
boost::optional<utility::string_t> sso{ utility::string_t(U("user@example.com")) };
boost::optional<bool> isCrawler{ false };
boost::optional<bool> includeUserInfo{ true };

api->getFeedPostsPublic(tenantId, afterId, limit, tags, sso, isCrawler, includeUserInfo)
.then([](pplx::task<std::shared_ptr<GetFeedPostsPublic_200_response>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<GetFeedPostsPublic_200_response>(*resp);
    } catch(const std::exception&) {
    }
});
[inline-code-end]
