---
## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| broadcastId | string | Nie |  |
| editKey | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[PublicAPIDeleteCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_delete_comment_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", editKey = "", sso = "")
if response.isSome:
  let deleted = response.get()
  echo "Delete acknowledged, HTTP status: ", httpResponse.status
[inline-code-end]

---