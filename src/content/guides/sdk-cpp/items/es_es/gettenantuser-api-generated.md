## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantUserResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
api->getTenantUser(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetTenantUserResponse>> task) {
        try {
            auto response = task.get();
            // Usar la respuesta según sea necesario
        } catch (const std::exception&) {
            // Manejo de errores
        }
    });
[inline-code-end]

---