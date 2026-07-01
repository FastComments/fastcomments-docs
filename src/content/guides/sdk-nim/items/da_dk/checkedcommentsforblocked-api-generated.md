## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | No |  |
| sso | string = "" | No |  |

## Svar

Returnerer: [`Option[CheckBlockedCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_check_blocked_comments_response.nim)

## Eksempel

[inline-code-attrs-start title = 'checkedCommentsForBlocked Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.checkedCommentsForBlocked(
  tenantId = "my-tenant-123",
  commentIds = "cmt-1,cmt-2",
  sso = ""
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
  discard response
[inline-code-end]

---