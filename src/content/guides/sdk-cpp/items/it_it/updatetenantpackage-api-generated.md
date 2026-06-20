---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio updateTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto packageId = utility::string_t(U("pkg-987"));
auto body = std::make_shared<UpdateTenantPackageBody>();
body->plan = utility::string_t(U("premium"));
body->seats = boost::optional<int>(50);
body->expiresAt = boost::optional<utility::string_t>(U("2026-12-31T23:59:59Z"));
body->contactEmail = boost::optional<utility::string_t>(U("ops@acme-corp.com"));
api->updateTenantPackage(tenantId, packageId, body)
.then([](std::shared_ptr<APIEmptyResponse> resp){
    if(!resp) throw std::runtime_error("Failed to update tenant package");
});
[inline-code-end]

---