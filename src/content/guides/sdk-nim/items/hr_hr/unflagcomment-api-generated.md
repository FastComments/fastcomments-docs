## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vraća: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## Primjer

[inline-code-attrs-start title = 'unFlagComment Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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