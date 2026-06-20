---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentIds | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[CheckBlockedCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_check_blocked_comments_response.nim)

## Primer

[inline-code-attrs-start title = 'checkedCommentsForBlocked Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.checkedCommentsForBlocked(
  tenantId = "my-tenant-123",
  commentIds = "",
  sso = ""
)
if response.isSome:
  let blockedResp = response.get()
  echo "Received blocked comments response: ", blockedResp
else:
  echo "No response body; HTTP status: ", $httpResponse.status
[inline-code-end]

---