## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`DeleteFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteFeedPostPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-9876");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-42"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto placeholder = std::make_shared<DeleteFeedPostPublic_200_response>();
api->deleteFeedPostPublic(tenantId, postId, broadcastId, sso)
.then([&placeholder](pplx::task<std::shared_ptr<DeleteFeedPostPublic_200_response>> task) {
    try {
        placeholder = task.get();
        if (placeholder) std::cout << "Feed post deleted for tenant\n";
        else std::cout << "DeleteFeedPostPublic returned no content\n";
    } catch (const std::exception& e) {
        std::cerr << "Delete failed: " << e.what() << '\n';
    }
});
[inline-code-end]
