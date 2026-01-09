---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentIds | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[CheckedCommentsForBlocked_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_checked_comments_for_blocked200response.nim)

## Eksempel

[inline-code-attrs-start title = 'checkedCommentsForBlocked Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.checkedCommentsForBlocked(
  tenantId = "my-tenant-123",
  commentIds = "",
  sso = ""
)
if response.isSome:
  let checked = response.get()
  echo "Checked comments received for tenant my-tenant-123"
  echo checked
else:
  echo "No checked comments (HTTP status: ", $httpResponse.statusCode, ")"
[inline-code-end]

---