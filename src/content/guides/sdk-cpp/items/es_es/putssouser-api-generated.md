## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Sí |  |
| updateComments | bool | No |  |

## Respuesta

Devuelve: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de putSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
UpdateAPISSOUserData updateData;
updateData.displayName = U("Jane Doe");
updateData.email = U("user@example.com");
boost::optional<bool> updateComments = true;
api->putSSOUser(tenantId, id, updateData, updateComments)
.then([](std::shared_ptr<PutSSOUserAPIResponse> resp){
    if(!resp){ std::cout << "putSSOUser returned null\n"; return; }
    auto copy = std::make_shared<PutSSOUserAPIResponse>(*resp);
    std::cout << "SSO user updated successfully\n";
});
[inline-code-end]

---