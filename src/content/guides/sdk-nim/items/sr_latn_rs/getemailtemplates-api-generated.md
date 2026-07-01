## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| skip | float64 | Ne |  |

## Odgovor

Vraća: [`Option[GetEmailTemplatesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates_response.nim)

## Primer

[inline-code-attrs-start title = 'getEmailTemplates Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (emailTemplatesOpt, httpResp) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if emailTemplatesOpt.isSome:
  let templates = emailTemplatesOpt.get()
  echo templates
[inline-code-end]