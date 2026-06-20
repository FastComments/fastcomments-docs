## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| contextUserId | string | No |  |
| isLive | bool | No |  |

## Respuesta

Devuelve: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentResult.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
boost::optional<utility::string_t> contextUserId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<bool> isLive = boost::optional<bool>(true);

api->deleteComment(tenantId, commentId, contextUserId, isLive)
    .then([](std::shared_ptr<DeleteCommentResult> result){
        auto res = result ? result : std::make_shared<DeleteCommentResult>();
        std::cout << "DeleteCommentResult ptr=" << static_cast<const void*>(res.get()) << std::endl;
    });
[inline-code-end]

---