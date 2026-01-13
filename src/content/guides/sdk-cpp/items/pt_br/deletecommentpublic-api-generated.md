## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| broadcastId | string | Sim |  |
| editKey | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`DeleteCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentPublic_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t commentId(U("cmt-456"));
utility::string_t broadcastId(U("brd-789"));
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(utility::string_t(U("editkey-abc123")));
boost::optional<utility::string_t> sso; 

api->deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso)
.then([](pplx::task<std::shared_ptr<DeleteCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copied = std::make_shared<DeleteCommentPublic_200_response>(*resp);
            (void)copied;
        }
    } catch (const std::exception&) {
        auto fallback = std::make_shared<DeleteCommentPublic_200_response>();
        (void)fallback;
    }
});
[inline-code-end]