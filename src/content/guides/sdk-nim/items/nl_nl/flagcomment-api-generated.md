---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nee |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |

## Respons

Retourneert: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'flagComment Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-98765", userId = "user-8342", anonUserId = "")
if response.isSome:
  let flagged = response.get()
  echo "Flagged comment response: ", flagged
else:
  echo "Flag comment failed: ", httpResponse
[inline-code-end]

---