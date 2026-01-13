---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| isFlagged | bool | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de flagCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-789012");
bool isFlagged = true;
boost::optional<utility::string_t> sso{ U("user@example.com") };
api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    auto respCopy = std::make_shared<FlagCommentPublic_200_response>(*resp);
    return respCopy;
});
[inline-code-end]

---