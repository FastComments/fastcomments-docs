## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostUnFlagCommentOptions& | Yes |  |

## Resposta

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemplo

[inline-code-attrs-start title = 'postUnFlagComment Exemplo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
PostUnFlagCommentOptions opts;
opts.notifyUser = boost::optional<bool>(true);
api->postUnFlagComment(tenantId, commentId, opts)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // o processamento pode ser feito aqui
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception&) {}
    });
[inline-code-end]