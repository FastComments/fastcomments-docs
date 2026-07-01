## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|-------------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Sí |  |
| options | const UnBlockUserFromCommentOptions& | Sí |  |

## Respuesta

Devuelve: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unBlockUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto params = std::make_shared<UnBlockFromCommentParams>();
params->commentId = U("cmt-12345");
params->reason = U("resolved");
UnBlockUserFromCommentOptions opts;
opts.notifyUser = boost::optional<bool>(true);
api->unBlockUserFromComment(U("my-tenant-123"), U("user-456"), *params, opts)
    .then([](pplx::task<std::shared_ptr<UnblockSuccess>> t){
        try{
            auto result = t.get();
        }catch(...){}
    });
[inline-code-end]

---