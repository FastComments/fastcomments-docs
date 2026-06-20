---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantBody | UpdateTenantBody | Ja |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'updateTenant Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("admin-user-456");
auto updateTenantBody = std::make_shared<UpdateTenantBody>();
updateTenantBody->name = U("Acme Corporation");
updateTenantBody->ownerEmail = boost::optional<utility::string_t>(U("owner@acme.com"));
updateTenantBody->isActive = boost::optional<bool>(true);
api->updateTenant(tenantId, id, updateTenantBody)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
    try {
        auto resp = t.get();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---