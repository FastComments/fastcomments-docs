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
utility::string_t commentId = U("cmt-456789");
utility::string_t broadcastId = U("broadcast-001");
CommentTextUpdateRequest req;
req.text = U("Thanks — updated comment text with additional context.");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("editKey-abc123"));
boost::optional<utility::string_t> sso = boost::none;
api->setCommentText(tenantId, commentId, broadcastId, req, editKey, sso)
.then([](std::shared_ptr<SetCommentText_200_response> resp){
    std::cout << "SetCommentText completed, response present: " << (resp ? "yes" : "no") << std::endl;
});
[inline-code-end]
