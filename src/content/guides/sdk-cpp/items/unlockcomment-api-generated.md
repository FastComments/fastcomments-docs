## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`LockComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/LockComment_200_response.h)

## Example

[inline-code-attrs-start title = 'unLockComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
utility::string_t broadcastId = U("broadcast-789");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

auto unlockTask = api->unLockComment(tenantId, commentId, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<LockComment_200_response>> prev) {
        try {
            auto resp = prev.get();
            if (!resp) resp = std::make_shared<LockComment_200_response>();
            return resp;
        } catch (...) {
            return std::make_shared<LockComment_200_response>();
        }
    });
[inline-code-end]
