---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| badgesUserId | string | No |  |
| commentId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserManualBadgesResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getManualBadgesForUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> badgesUserId(boost::optional<utility::string_t>(U("user@example.com")));
boost::optional<utility::string_t> commentId(boost::optional<utility::string_t>(U("cmt-789")));
boost::optional<utility::string_t> sso(boost::optional<utility::string_t>(U("my-tenant-123|sso-token-abc")));
auto task = api->getManualBadgesForUser(badgesUserId, commentId, sso)
.then([](pplx::task<std::shared_ptr<GetUserManualBadgesResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetUserManualBadgesResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetUserManualBadgesResponse>();
    }
});
[inline-code-end]

---