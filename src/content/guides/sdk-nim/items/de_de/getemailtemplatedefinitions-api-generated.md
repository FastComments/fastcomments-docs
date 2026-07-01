## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|--------------|
| tenantId | string | Ja |  |

## Antwort

Rückgabe: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if responseOpt.isSome:
  let definitions = responseOpt.get()
  echo definitions
[inline-code-end]

---