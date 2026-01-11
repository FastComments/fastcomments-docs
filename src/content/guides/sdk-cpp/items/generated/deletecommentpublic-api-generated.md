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
utility::string_t commentId = U("cmt-7890");
utility::string_t broadcastId = U("bcast-456");
boost::optional<utility::string_t> editKey = utility::string_t(U("editKey-abc123"));
boost::optional<utility::string_t> sso = boost::none;

api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](std::shared_ptr<DeleteCommentPublic_200_response> resp) {
    auto copy = std::make_shared<DeleteCommentPublic_200_response>(*resp);
    (void)copy;
});
[inline-code-end]
