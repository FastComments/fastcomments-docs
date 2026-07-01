## Parâmetros

| Name | Type | Required | Descrição |
|------|------|----------|-----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostRemoveCommentOptions | No |  |

## Resposta

Retorna: [`Option[PostRemoveCommentApiResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_post_remove_comment_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'postRemoveComment Exemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponseOpt, httpResp) = client.postRemoveComment(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  options = PostRemoveCommentOptions()
)

if apiResponseOpt.isSome:
  let apiResponse = apiResponseOpt.get()
[inline-code-end]