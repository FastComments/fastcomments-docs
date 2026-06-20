---
Información masiva de usuarios para un tenant. Dado userIds, devuelve información para mostrar de User / SSOUser.
Usado por el widget de comentarios para enriquecer usuarios que acaban de aparecer mediante un evento de presencia.
Sin contexto de página: la privacidad se aplica de forma uniforme (los perfiles privados están enmascarados).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'Ejemplo de getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> statusFilter = U("active");
api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try {
        auto res = t.get();
        if (res) {
            auto responseCopy = std::make_shared<PageUsersInfoResponse>(*res);
        }
    } catch (const std::exception&) {}
});
[inline-code-end]

---