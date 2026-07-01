## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| skip | float64 | Non |  |

## Réponse

Retourne : [`Option[GetEmailTemplatesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getEmailTemplates'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (emailTemplatesOpt, httpResp) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if emailTemplatesOpt.isSome:
  let templates = emailTemplatesOpt.get()
  echo templates
[inline-code-end]