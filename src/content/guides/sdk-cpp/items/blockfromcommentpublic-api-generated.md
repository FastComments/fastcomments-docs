## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`BlockFromCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockFromCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'blockFromCommentPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654");
auto paramsPtr = std::make_shared<PublicBlockFromCommentParams>();
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->blockFromCommentPublic(tenantId, commentId, *paramsPtr, sso)
.then([](std::shared_ptr<BlockFromCommentPublic_200_response> resp){
    (void)resp;
});
[inline-code-end]
