---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantPackageResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("pkg-basic-001");
boost::optional<utility::string_t> requestedBy = U("admin@example.com");
auto task = api->getTenantPackage(tenantId, id)
    .then([requestedBy](std::shared_ptr<GetTenantPackageResponse> resp) -> std::shared_ptr<GetTenantPackageResponse> {
        if(!resp) return std::make_shared<GetTenantPackageResponse>();
        if(requestedBy) {}
        return std::make_shared<GetTenantPackageResponse>(*resp);
    });
[inline-code-end]

---