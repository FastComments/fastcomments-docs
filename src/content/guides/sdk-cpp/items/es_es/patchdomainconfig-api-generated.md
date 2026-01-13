## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| domainToUpdate | string | Sí |  |
| patchDomainConfigParams | PatchDomainConfigParams | Sí |  |

## Respuesta

Devuelve: [`GetDomainConfig_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetDomainConfig_200_response.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de patchDomainConfig'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domainToUpdate = U("example.com");
PatchDomainConfigParams params;
params.adminEmail = boost::optional<utility::string_t>(U("admin@example.com"));
params.enableSso = boost::optional<bool>(true);
params.ssoRedirectUrl = boost::optional<utility::string_t>(U("https://auth.example.com/callback"));
api->patchDomainConfig(tenantId, domainToUpdate, params)
    .then([](pplx::task<std::shared_ptr<GetDomainConfig_200_response>> task){
        try {
            auto resp = task.get();
            if (resp) {
                auto updated = std::make_shared<GetDomainConfig_200_response>(*resp);
            }
        } catch (const std::exception& e) {
            auto err = std::string(e.what());
        }
    });
[inline-code-end]

---