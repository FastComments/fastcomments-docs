## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Gibt zurück: [`Option[GetEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getEmailTemplate Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email-01")
if response.isSome:
  let template = response.get()
  discard template
[inline-code-end]

---