## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| voteBodyParams | VoteBodyParams | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`VoteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteComment_200_response.h)

## Example

[inline-code-attrs-start title = 'voteComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t urlId = U("url-9a8b7c");
utility::string_t broadcastId = U("broadcast-2025-01-09");
auto voteBody = std::make_shared<VoteBodyParams>();
boost::optional<utility::string_t> sessionId(U("session-0a1b2c"));
boost::optional<utility::string_t> sso(U("user@example.com"));
api->voteComment(tenantId, commentId, urlId, broadcastId, *voteBody, sessionId, sso)
    .then([](pplx::task<std::shared_ptr<VoteComment_200_response>> task){
        try {
            auto resp = task.get();
            (void)resp;
        } catch(const std::exception &){
        }
    });
[inline-code-end]
