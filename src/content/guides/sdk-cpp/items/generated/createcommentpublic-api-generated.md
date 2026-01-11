## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentData | CommentData | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'createCommentPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("articles/how-to-cpprest");
utility::string_t broadcastId = U("broadcast-987");
auto commentDataPtr = std::make_shared<CommentData>();
commentDataPtr->text = U("Great article! Learned a lot.");
commentDataPtr->authorEmail = U("reader@example.com");
boost::optional<utility::string_t> sessionId = boost::optional<utility::string_t>(U("session-abc-123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-456"));
api->createCommentPublic(tenantId, urlId, broadcastId, *commentDataPtr, sessionId, sso)
.then([](std::shared_ptr<CreateCommentPublic_200_response> resp){
    if (resp) {}
});
[inline-code-end]
