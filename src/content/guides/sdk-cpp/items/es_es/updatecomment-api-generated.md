## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updatableCommentParams | UpdatableCommentParams | Sí |  |
| options | const UpdateCommentOptions& | Sí |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo updateComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
auto commentId = utility::string_t("cmt-789");
UpdatableCommentParams updatable;
updatable.body = utility::string_t("Edited comment content");
updatable.isSpam = boost::optional<bool>(false);
UpdateCommentOptions options;
options.notify = boost::optional<bool>(true);
api->updateComment(tenantId, commentId, updatable, options)
    .then([](std::shared_ptr<APIEmptyResponse>) {})
    .then([](pplx::task<void> t) { try { t.get(); } catch(const std::exception&) {} });
[inline-code-end]