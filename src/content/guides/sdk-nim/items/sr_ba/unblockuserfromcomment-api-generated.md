## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Ne |  |
| options | UnBlockUserFromCommentOptions | Ne |  |

## Odgovor

Vraća: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Primjer

[inline-code-attrs-start title = 'unBlockUserFromComment Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  unBlockFromCommentParams = UnBlockFromCommentParams(userId = "user-789", commentId = "cmt-321"),
  options = UnBlockUserFromCommentOptions(),
)

if response.isSome:
  let unblockSuccess = response.get()
[inline-code-end]