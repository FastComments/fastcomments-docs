## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`BlockFromCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockFromCommentPublic_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de blockFromCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
PublicBlockFromCommentParams params;
params.reason = U("Repeated harassment");
params.durationMinutes = 1440;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc123"));
auto placeholder = std::make_shared<BlockFromCommentPublic_200_response>();
api->blockFromCommentPublic(tenantId, commentId, params, sso)
.then([](pplx::task<std::shared_ptr<BlockFromCommentPublic_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) std::wcout << U("Comment blocked successfully\n");
        else std::wcout << U("Block request returned empty response\n");
    } catch (...) {
        std::wcout << U("Block request failed\n");
    }
});
[inline-code-end]

---