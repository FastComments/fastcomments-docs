---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| skip | double | Nein |  |

## Antwort

Gibt zurück: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("email-template-789");
boost::optional<double> skip = boost::optional<double>(10.0);
api->getEmailTemplateRenderErrors(tenantId, templateId, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> t) {
        try {
            auto resp = t.get();
            auto safeResp = resp ? resp : std::make_shared<GetEmailTemplateRenderErrorsResponse>();
            (void)safeResp;
        } catch (const std::exception& e) {
            (void)e;
        }
    }).wait();
[inline-code-end]

---