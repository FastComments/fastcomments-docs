## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo blockFromCommentPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-987654");
PublicBlockFromCommentParams blockParams;
blockParams.reason = utility::conversions::to_string_t("spam");
blockParams.durationHours = 24;
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("sso-token-abc123");
api->blockFromCommentPublic(tenantId, commentId, blockParams, sso)
    .then([](std::shared_ptr<BlockSuccess> result){
        auto successCopy = std::make_shared<BlockSuccess>(*result);
    });
[inline-code-end]