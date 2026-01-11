## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UnBlockCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnBlockCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'unBlockCommentPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
PublicBlockFromCommentParams publicBlockFromCommentParams;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("sso-token-abc123")));

api->unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso)
.then([](pplx::task<std::shared_ptr<UnBlockCommentPublic_200_response>> t) {
    try {
        auto response = t.get();
        if (!response) {
            response = std::make_shared<UnBlockCommentPublic_200_response>();
        }
    } catch (const std::exception& e) {
    }
});
[inline-code-end]
