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
utility::string_t commentId = U("cmt-987654");
utility::string_t broadcastId = U("brd-456");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto dummyResp = std::make_shared<LockComment_200_response>();
api->unLockComment(tenantId, commentId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<LockComment_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            std::cout << "Comment unlocked successfully\n";
        } else {
            std::cout << "Unlock returned empty response\n";
        }
    } catch (const std::exception& e) {
        std::cerr << "Error unlocking comment: " << e.what() << '\n';
    }
});
[inline-code-end]