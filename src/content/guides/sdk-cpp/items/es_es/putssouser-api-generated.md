## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Sí |  |
| updateComments | bool | No |  |

## Respuesta

Devuelve: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PutSSOUserAPIResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo putSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
UpdateAPISSOUserData userData;
userData.email = utility::conversions::to_string_t("alice@example.com");
userData.first_name = utility::conversions::to_string_t("Alice");
userData.last_name = utility::conversions::to_string_t("Smith");
userData.role = utility::conversions::to_string_t("moderator");

api->putSSOUser(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("alice.smith"),
    userData,
    boost::optional<bool>(true)
).then([](pplx::task<std::shared_ptr<PutSSOUserAPIResponse>> t) {
    auto response = t.get();
});
[inline-code-end]

---