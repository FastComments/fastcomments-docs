Información de usuarios en bloque para un inquilino. Dado **userIds**, devuelve información de visualización de **User / SSOUser**.  
Usado por el widget de comentarios para enriquecer a los usuarios que acaban de aparecer mediante un evento de presencia.  
Sin contexto de página: la privacidad se aplica de manera uniforme (los perfiles privados están enmascarados).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| ids | string | Sí |  |

## Response

Devuelve: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Example

[inline-code-attrs-start title = 'getUsersInfo Ejemplo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // procesar respuesta
    }catch(const std::exception&){
        // manejar error
    }
});
[inline-code-end]