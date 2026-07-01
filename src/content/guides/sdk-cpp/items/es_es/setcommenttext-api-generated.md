---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|--------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Sí |  |
| options | const SetCommentTextOptions& | Sí |  |

## Respuesta

Devuelve: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPISetCommentTextResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo setCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto commentId = U("cmt-456");
auto broadcastId = U("brd-789");

CommentTextUpdateRequest updateReq;
updateReq.text = U("Updated comment content");
updateReq.isEdited = boost::make_optional(true);

SetCommentTextOptions opts;
opts.notifySubscribers = boost::make_optional(true);

api->setCommentText(tenantId, commentId, broadcastId, updateReq, opts)
    .then([](std::shared_ptr<PublicAPISetCommentTextResponse> resp) {
    });
[inline-code-end]

---