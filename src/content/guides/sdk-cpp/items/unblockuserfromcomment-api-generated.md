## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`UnBlockCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnBlockCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'unBlockUserFromComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
auto params = std::make_shared<UnBlockFromCommentParams>();
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> anonUserId(U("anon-abc-123"));
api->unBlockUserFromComment(tenantId, commentId, params, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<UnBlockCommentPublic_200_response>> task){
    try {
        auto response = task.get();
    } catch (const std::exception&){
    }
});
[inline-code-end]
