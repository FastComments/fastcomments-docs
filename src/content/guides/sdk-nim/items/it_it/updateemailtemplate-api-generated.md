---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | No |  |

## Risposta

Restituisce: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateEmailTemplateBody(
  subject = "Welcome to Newsly",
  html = "<p>Thanks for joining Newsly! Visit https://newsly.example to get started.</p>",
  fromAddress = "no-reply@newsly.example",
  fromName = "Newsly Team",
  enabled = true
)
let (response, httpResponse) = client.updateEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email", updateEmailTemplateBody = updateBody)
if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse.statusCode
[inline-code-end]

---