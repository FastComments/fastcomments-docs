## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'unPinComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unPinComment(tenantId = "my-tenant-123", commentId = "cmt-987654321", broadcastId = "", sso = "")
if response.isSome:
  let result = response.get()
  echo "Unpinned comment:", $result
else:
  echo "Unpin failed, HTTP status:", $httpResponse.status
[inline-code-end]

---