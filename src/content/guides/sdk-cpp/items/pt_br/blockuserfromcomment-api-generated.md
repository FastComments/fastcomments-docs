## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| blockFromCommentParams | BlockFromCommentParams | Sim |  |
| options | const BlockUserFromCommentOptions& | Sim |  |

## Resposta

Retorna: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo blockUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-789");
BlockFromCommentParams params;
params.reason = utility::conversions::to_string_t("Inappropriate content");
params.durationDays = boost::optional<int>(30);
BlockUserFromCommentOptions options;
options.notifyUser = boost::optional<bool>(true);

api->blockUserFromComment(tenantId, commentId, params, options)
    .then([](std::shared_ptr<BlockSuccess> result){ })
    .then([](pplx::task<void> t){ try { t.get(); } catch (const std::exception&) { } });
[inline-code-end]