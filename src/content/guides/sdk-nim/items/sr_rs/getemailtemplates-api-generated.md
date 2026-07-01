## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## Odgovor

Vraća: [`Option[GetEmailTemplatesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getEmailTemplates'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (emailTemplatesOpt, httpResp) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if emailTemplatesOpt.isSome:
  let templates = emailTemplatesOpt.get()
  echo templates
[inline-code-end]