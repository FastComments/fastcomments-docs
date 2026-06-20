## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Нет |  |
| sso | string | Нет |  |

## Response

Возвращает: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Пример

[inline-code-attrs-start title = 'Пример unBlockCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", publicBlockFromCommentParams = PublicBlockFromCommentParams(), sso = "")
if response.isSome:
  let unblockResult = response.get()
  discard unblockResult
else:
  discard httpResponse
[inline-code-end]

---