## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createTenantBody | CreateTenantBody | Sí |  |

## Respuesta

Devuelve: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateTenantResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateTenantBody>();
bodyPtr->setName(utility::string_t(U("Acme Corporation")));
bodyPtr->setAdminEmail(utility::string_t(U("admin@acme.com")));
bodyPtr->setSupportEmail(boost::optional<utility::string_t>(utility::string_t(U("support@acme.com"))));
bodyPtr->setPlan(boost::optional<utility::string_t>(utility::string_t(U("pro"))));
api->createTenant(tenantId, *bodyPtr).then([](pplx::task<std::shared_ptr<CreateTenantResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto newTenantId = resp->getTenantId();
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---