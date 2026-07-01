## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| skip | double | No |  |

## Réponse

Renvoie : [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## Exemple

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Exemple'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip = 20.0;

api->getEmailTemplateRenderErrors(tenantId, id, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> task) {
        try {
            auto response = task.get();
            // Utilisez la réponse selon vos besoins
        } catch (const std::exception& ex) {
            // Gérer l'erreur
        }
    });
[inline-code-end]