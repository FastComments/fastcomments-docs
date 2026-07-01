## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetEmailTemplateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateResponse.h)

## Esempio

[inline-code-attrs-start title = 'getEmailTemplate Esempio'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto templateId = utility::conversions::to_string_t("welcome-email");
boost::optional<utility::string_t> language = utility::conversions::to_string_t("en-US");

api->getEmailTemplate(tenantId, templateId)
    .then([=](pplx::task<std::shared_ptr<GetEmailTemplateResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]