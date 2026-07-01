## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| setCommentTextParams | SetCommentTextParams | No |  |
| options | PostSetCommentTextOptions | No |  |

## Respuesta

Returns: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'postSetCommentText Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.postSetCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  setCommentTextParams = SetCommentTextParams(),
  options = PostSetCommentTextOptions()
)

if responseOpt.isSome:
  let updatedComment = responseOpt.get()
[inline-code-end]