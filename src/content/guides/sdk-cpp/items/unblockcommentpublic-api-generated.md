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
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t commentId = utility::conversions::to_string_t("cmt-456");
auto paramsPtr = std::make_shared<PublicBlockFromCommentParams>();
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("user@example.com"));
api->unBlockCommentPublic(tenantId, commentId, *paramsPtr, sso)
.then([](std::shared_ptr<UnBlockCommentPublic_200_response> resp) {
    (void)resp;
    return resp;
});
[inline-code-end]
