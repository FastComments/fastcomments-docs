## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Oui |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de mise à jour du modèle d\'email'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email");
UpdateEmailTemplateBody body;
body.subject = U("Welcome to Our Platform");
body.content = U("<p>Hello \{{userName}}, welcome aboard!</p>");
body.isActive = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, body)
    .then([](std::shared_ptr<APIEmptyResponse> response) {
        // gestion du succès
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception &) { /* gestion des erreurs */ }
    });
[inline-code-end]