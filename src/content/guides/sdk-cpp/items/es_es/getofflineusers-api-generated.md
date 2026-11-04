Comentaristas anteriores en la página que NO están actualmente en línea. Ordenados por displayName.
Úselo después de agotar /users/online para mostrar una sección "Members".
Paginación por cursor en commenterName: el servidor recorre el índice parcial {tenantId, urlId, commenterName}
desde afterName hacia adelante mediante $gt, sin coste de $skip.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| options | const GetOfflineUsersOptions& | Sí |  |

## Respuesta

Devuelve: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]