---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |

## Réponse

Retourne : [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getEmailTemplateDefinitions'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if response.isSome:
  let definitions = response.get()
  echo "Email template definitions for my-tenant-123: ", definitions
else:
  echo "Failed to retrieve templates, HTTP status: ", httpResponse.status
[inline-code-end]

---