## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostRemoveCommentOptions& | Yes |  |

## Resposta

Retorna: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PostRemoveCommentApiResponse.h)

## Exemplo

[inline-code-attrs-start title = 'postRemoveComment Exemplo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("cmt-456789"));
PostRemoveCommentOptions options;
options.permanent = boost::optional<bool>(true);
api->postRemoveComment(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentApiResponse>> task) {
        try {
            auto response = task.get();
            // Processar resposta
        } catch (const std::exception& ex) {
            // Tratar erro
        }
    });
[inline-code-end]

---