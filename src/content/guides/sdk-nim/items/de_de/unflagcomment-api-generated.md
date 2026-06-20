---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Beispiel

[inline-code-attrs-start title = 'unFlagComment Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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