## Parámetros

| Name | Type | Obligatorio | Descripción |
|------|------|-------------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenantPackage'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t packageId = U("pkg-456");
auto bodyPtr = std::make_shared<UpdateTenantPackageBody>();
bodyPtr->name = utility::string_t(U("Pro Monthly"));
bodyPtr->description = boost::optional<utility::string_t>(U("Monthly subscription for pro users"));
bodyPtr->enabled = boost::optional<bool>(true);
api->updateTenantPackage(tenantId, packageId, *bodyPtr)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    std::cout << (resp ? "updateTenantPackage succeeded\n" : "updateTenantPackage returned null\n");
});
[inline-code-end]

---