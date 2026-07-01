## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| skip | double | Ne |  |

## Response

Vrne: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## Example

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip = 20.0;

api->getEmailTemplateRenderErrors(tenantId, id, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> task) {
        try {
            auto response = task.get();
            // Uporabite odziv po potrebi
        } catch (const std::exception& ex) {
            // Obdelajte napako
        }
    });
[inline-code-end]

---