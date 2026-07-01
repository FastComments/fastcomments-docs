## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| sendEmail | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteModerator'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto moderatorId = utility::conversions::to_string_t("mod-456");
boost::optional<utility::string_t> sendEmail = utility::conversions::to_string_t("admin@example.com");
api->deleteModerator(tenantId, moderatorId, sendEmail)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        // tratar sucesso
    });
[inline-code-end]