## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
auto updateParams = std::make_shared<UpdateFeedPostParams>();
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->updateFeedPostPublic(tenantId, postId, *updateParams, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostPublic_200_response>> task) {
    try {
        auto response = task.get();
        (void)response;
    } catch (const std::exception &e) {
        throw;
    }
});
[inline-code-end]
