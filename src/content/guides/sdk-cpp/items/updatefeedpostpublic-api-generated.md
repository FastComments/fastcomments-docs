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
utility::string_t postId = U("post-98765");
UpdateFeedPostParams updateFeedPostParams = UpdateFeedPostParams();
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-444"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto responseHolder = std::make_shared<CreateFeedPostPublic_200_response>();
api->updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<CreateFeedPostPublic_200_response>> t) {
        try {
            auto resp = t.get();
            (void)resp;
        } catch (const std::exception&) {}
    });
[inline-code-end]
