## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateModeratorBody | UpdateModeratorBody | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateModerator'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto moderatorId = utility::conversions::to_string_t("mod-789");
UpdateModeratorBody body;
body.email = utility::conversions::to_string_t("moderator@example.com");
body.isActive = true;
body.notes = boost::optional<utility::string_t>(utility::conversions::to_string_t("Senior moderator"));
api->updateModerator(tenantId, moderatorId, body)
    .then([](std::shared_ptr<APIEmptyResponse>) {})
    .then([](pplx::task<void> t) { try { t.get(); } catch (const std::exception&) {} });
[inline-code-end]