## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | double | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackagesResponse.h)

## Przykład

[inline-code-attrs-start title = 'getTenantPackages Przykład'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> skip = 20.0;
api->getTenantPackages(tenantId, skip)
    .then([](std::shared_ptr<GetTenantPackagesResponse> resp) {
        (void)resp;
    });
[inline-code-end]

---