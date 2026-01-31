## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'sendInvite Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
utility::string_t fromName = U("moderator@acmecorp.com");
boost::optional<utility::string_t> inviteNote = boost::optional<utility::string_t>(U("Please review this comment"));

api->sendInvite(tenantId, commentId, fromName)
.then([=](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        auto result = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
        return result;
    } catch (const std::exception&) {
        return std::make_shared<FlagCommentPublic_200_response>();
    }
});
[inline-code-end]
