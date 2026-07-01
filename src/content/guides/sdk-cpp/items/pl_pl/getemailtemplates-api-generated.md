## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Odpowiedź

Zwraca: [`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplatesResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład getEmailTemplates'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> skip = 10.0;
api->getEmailTemplates(tenantId, skip)
    .then([](std::shared_ptr<GetEmailTemplatesResponse> resp) {
        (void)resp;
    });
[inline-code-end]