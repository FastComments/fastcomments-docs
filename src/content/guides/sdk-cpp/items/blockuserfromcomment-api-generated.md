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
utility::string_t commentId = U("cmt-987654321");
BlockFromCommentParams params;
params.reason = U("Repeated abusive language");
boost::optional<utility::string_t> userIdOpt = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserIdOpt;
api->blockUserFromComment(tenantId, commentId, params, userIdOpt, anonUserIdOpt)
    .then([](pplx::task<std::shared_ptr<BlockFromCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<BlockFromCommentPublic_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
