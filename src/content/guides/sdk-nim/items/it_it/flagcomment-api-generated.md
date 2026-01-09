## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Risposta

Restituisce: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di flagComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-98765", userId = "user-8342", anonUserId = "")
if response.isSome:
  let flagged = response.get()
  echo "Flagged comment response: ", flagged
else:
  echo "Flag comment failed: ", httpResponse
[inline-code-end]

---