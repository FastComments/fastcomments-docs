## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| blockFromCommentParams | BlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`BlockFromCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockFromCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'blockUserFromComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
BlockFromCommentParams params;
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::optional<utility::string_t>(U("anon-abc-123"));
api->blockUserFromComment(tenantId, commentId, params, userId, anonUserId)
.then([](std::shared_ptr<BlockFromCommentPublic_200_response> resp){
    if(resp){
        auto wrapper = std::make_shared<std::shared_ptr<BlockFromCommentPublic_200_response>>(resp);
        std::cout << U("Blocked user from comment successfully\n");
    }
})
.wait();
[inline-code-end]
