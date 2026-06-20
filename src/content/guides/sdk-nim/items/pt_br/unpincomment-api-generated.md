## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de unPinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unPinComment(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", sso = "")
if response.isSome:
  let result = response.get()
  echo "Unpinned comment:", $result
else:
  echo "Unpin failed, HTTP status:", $httpResponse.status
[inline-code-end]

---