## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|------------|-------------|
| tag | string | Sí |  |
| tenantId | string | No |  |
| updateHashTagBody | UpdateHashTagBody | No |  |

## Respuesta

Devuelve: [`PatchHashTag_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchHashTag_200_response.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de patchHashTag'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tag = U("release-1.0");
boost::optional<utility::string_t> tenantId = boost::optional<utility::string_t>(U("my-tenant-123"));
UpdateHashTagBody body;
boost::optional<UpdateHashTagBody> updateBody = boost::optional<UpdateHashTagBody>(body);
api->patchHashTag(tag, tenantId, updateBody)
.then([](pplx::task<std::shared_ptr<PatchHashTag_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto copied = std::make_shared<PatchHashTag_200_response>(*resp);
            std::cout << "PatchHashTag succeeded" << std::endl;
        } else {
            std::cout << "PatchHashTag returned no data" << std::endl;
        }
    } catch (const std::exception &e) {
        std::cerr << "PatchHashTag error: " << e.what() << std::endl;
    }
});
[inline-code-end]