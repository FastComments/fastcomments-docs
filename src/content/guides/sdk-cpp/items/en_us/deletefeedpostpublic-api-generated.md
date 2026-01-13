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
utility::string_t postId = U("post-456");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->deleteFeedPostPublic(tenantId, postId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<DeleteFeedPostPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<DeleteFeedPostPublic_200_response>();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---