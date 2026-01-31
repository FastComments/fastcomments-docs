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

[inline-code-attrs-start title = 'lockComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("brd-98765");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->lockComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<LockComment_200_response>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<LockComment_200_response>(*resp);
        }
    } catch (const std::exception &e) {
    }
});
[inline-code-end]
