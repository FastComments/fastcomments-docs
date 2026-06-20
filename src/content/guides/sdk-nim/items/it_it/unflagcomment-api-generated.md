## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di unFlagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unFlagComment(tenantId = "my-tenant-123",
  id = "comment-98765",
  userId = "user-12345",
  anonUserId = "")

if response.isSome:
  let flagResp = response.get()
  echo "Unflagged comment response:", flagResp
else:
  echo "Unflag failed, HTTP status:", httpResponse.status
[inline-code-end]

---