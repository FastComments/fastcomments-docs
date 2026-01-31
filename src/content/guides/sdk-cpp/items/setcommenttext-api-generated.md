## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`SetCommentText_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentText_200_response.h)

## Example

[inline-code-attrs-start title = 'setCommentText Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654");
utility::string_t broadcastId = U("bcast-2024-01");
CommentTextUpdateRequest commentTextUpdateRequest;
commentTextUpdateRequest.text = U("Updated comment text to correct typo and clarify meaning");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("edit-key-abc123"));
boost::optional<utility::string_t> sso;
api->setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso)
.then([](std::shared_ptr<SetCommentText_200_response> resp){
    (void)resp;
    return resp;
});
[inline-code-end]
