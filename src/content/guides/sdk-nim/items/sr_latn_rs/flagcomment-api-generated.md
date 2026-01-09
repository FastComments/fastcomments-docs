## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Odgovor

Vraća: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Primer

[inline-code-attrs-start title = 'flagComment Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-98765", userId = "user-8342", anonUserId = "")
if response.isSome:
  let flagged = response.get()
  echo "Flagged comment response: ", flagged
else:
  echo "Flag comment failed: ", httpResponse
[inline-code-end]

---