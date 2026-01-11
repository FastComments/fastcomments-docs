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
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
utility::string_t commentId = utility::string_t(U("comment-456"));
PublicBlockFromCommentParams params;
boost::optional<utility::string_t> sso = utility::string_t(U("sso-token-abc123"));
api->blockFromCommentPublic(tenantId, commentId, params, sso)
.then([](pplx::task<std::shared_ptr<BlockFromCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<BlockFromCommentPublic_200_response>();
        std::cout << "BlockFromCommentPublic completed\n";
    } catch (const std::exception& e) {
        std::cerr << "BlockFromCommentPublic error: " << e.what() << '\n';
    }
});
[inline-code-end]
