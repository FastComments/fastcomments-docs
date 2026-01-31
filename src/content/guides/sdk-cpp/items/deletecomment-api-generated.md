## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| contextUserId | string | No |  |
| isLive | bool | No |  |

## Response

Returns: [`DeleteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteComment_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t commentId(U("cmt-7890"));
boost::optional<utility::string_t> contextUserId(boost::optional<utility::string_t>(utility::string_t(U("user@example.com"))));
boost::optional<bool> isLive(boost::optional<bool>(true));
auto marker = std::make_shared<int>(0);
api->deleteComment(tenantId, commentId, contextUserId, isLive)
    .then([marker](std::shared_ptr<DeleteComment_200_response> resp) {
        if (resp)
            std::cout << "Comment deleted successfully\n";
        else
            std::cout << "Delete returned no data\n";
    });
[inline-code-end]
