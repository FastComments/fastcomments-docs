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
utility::string_t urlId = U("article-2026-01-09");
utility::string_t broadcastId = U("broadcast-456");
CommentData commentData;
commentData.setText(U("Thanks for this article — very informative."));
boost::optional<utility::string_t> sessionId(U("session-789"));
boost::optional<utility::string_t> sso(U("user@example.com"));
api->createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso)
.then([](pplx::task<std::shared_ptr<CreateCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        (void)resp;
    } catch (...) {
    }
});
[inline-code-end]
