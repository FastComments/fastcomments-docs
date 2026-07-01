## Parametry

| Nazwa | Typ | Wymagane | Opis |
|-------|-----|----------|------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| skip | double | Nie |  |

## Odpowiedź

Zwraca: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrorsResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getEmailTemplateRenderErrors'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip = 20.0;

api->getEmailTemplateRenderErrors(tenantId, id, skip)
    .then([](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrorsResponse>> task) {
        try {
            auto response = task.get();
            // Użyj odpowiedzi w razie potrzeby
        } catch (const std::exception& ex) {
            // Obsłuż błąd
        }
    });
[inline-code-end]