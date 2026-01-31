## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`DeleteCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteCommentPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("broadcast-987");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("editkey-abc123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<DeleteCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto resultCopy = std::make_shared<DeleteCommentPublic_200_response>(*resp);
        }
    } catch (const std::exception &e) {
    }
});
[inline-code-end]
