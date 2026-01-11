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
boost::optional<utility::string_t> sso(U("sso-token-abc123"));
api->unLockComment(U("my-tenant-123"), U("cmt-456"), U("broadcast-789"), sso)
.then([](pplx::task<std::shared_ptr<LockComment_200_response>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<LockComment_200_response>();
        (void)result;
    } catch (...) {
    }
});
[inline-code-end]
