## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetEmailTemplate_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplate_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-001");
boost::optional<utility::string_t> preferLocale = boost::optional<utility::string_t>(U("en-US"));
api->getEmailTemplate(tenantId, templateId)
    .then([preferLocale](std::shared_ptr<GetEmailTemplate_200_response> resp) {
        auto templateResp = resp ? resp : std::make_shared<GetEmailTemplate_200_response>();
        if (preferLocale) {
            (void)preferLocale;
        }
        return templateResp;
    });
[inline-code-end]

---