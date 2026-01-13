## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | double | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantUsers_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUsers_200_response.h)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getTenantUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = boost::optional<double>(20);
api->getTenantUsers(tenantId, skip)
    .then([=](pplx::task<std::shared_ptr<GetTenantUsers_200_response>> t) {
        try {
            std::shared_ptr<GetTenantUsers_200_response> resp = t.get();
            if (!resp) resp = std::make_shared<GetTenantUsers_200_response>();
            (void)resp;
        } catch (const std::exception& ex) {
            (void)ex;
            std::shared_ptr<GetTenantUsers_200_response> err = std::make_shared<GetTenantUsers_200_response>();
        }
    });
[inline-code-end]

---