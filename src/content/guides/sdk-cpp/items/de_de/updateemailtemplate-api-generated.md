## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Ja |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'updateEmailTemplate Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email");
UpdateEmailTemplateBody body;
body.subject = U("Welcome to Our Platform");
body.content = U("<p>Hello \{{userName}}, welcome aboard!</p>");
body.isActive = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, body)
    .then([](std::shared_ptr<APIEmptyResponse> response) {
        // Erfolgsbehandlung
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception &) { /* Fehlerbehandlung */ }
    });
[inline-code-end]