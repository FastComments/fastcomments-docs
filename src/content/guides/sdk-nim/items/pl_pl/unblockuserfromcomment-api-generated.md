## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| id | string | Nie |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Nie |  |
| options | UnBlockUserFromCommentOptions | Nie |  |

## Odpowiedź

Zwraca: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład unBlockUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---