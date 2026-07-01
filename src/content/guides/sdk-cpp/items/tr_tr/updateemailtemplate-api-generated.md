---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Örnek

[inline-code-attrs-start title = 'updateEmailTemplate Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email");
UpdateEmailTemplateBody body;
body.subject = U("Welcome to Our Platform");
body.content = U("<p>Hello \{{userName}}, welcome aboard!</p>");
body.isActive = boost::optional<bool>(true);
api->updateEmailTemplate(tenantId, templateId, body)
    .then([](std::shared_ptr<APIEmptyResponse> response) {
        // başarı işleme
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception &) { /* hata işleme */ }
    });
[inline-code-end]

---