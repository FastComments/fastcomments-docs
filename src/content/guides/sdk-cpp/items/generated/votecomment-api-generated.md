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
VoteBodyParams voteBodyParams;
boost::optional<utility::string_t> sessionId(U("sess-7f8e-7a9b"));
boost::optional<utility::string_t> sso(U("user@example.com"));
api->voteComment(U("my-tenant-123"), U("cmt-98765"), U("url-456"), U("br-222"), voteBodyParams, sessionId, sso)
.then([](pplx::task<std::shared_ptr<VoteComment_200_response>> task){
    try {
        auto resp = task.get();
        auto ack = std::make_shared<utility::string_t>(U("vote-recorded"));
        (void)ack;
    } catch(...) {
    }
});
[inline-code-end]
