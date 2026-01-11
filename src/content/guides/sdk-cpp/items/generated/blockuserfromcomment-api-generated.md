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
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
BlockFromCommentParams params;
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId = boost::none;
api->blockUserFromComment(tenantId, commentId, params, userId, anonUserId)
    .then([](pplx::task<std::shared_ptr<BlockFromCommentPublic_200_response>> t){
        try {
            auto resp = t.get();
            if (resp) {
                auto copy = std::make_shared<BlockFromCommentPublic_200_response>(*resp);
            }
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
