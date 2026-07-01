## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| skip | float64 | Non |  |

## Réponse

Renvoie : [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## Exemple

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Exemple'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getEmailTemplateRenderErrors(
  tenantId = "my-tenant-123",
  id = "welcome-template",
  skip = 0.0
)

if optResp.isSome:
  let resp = optResp.get()
  # utiliser la réponse selon les besoins
else:
  # gérer la réponse manquante
  discard
[inline-code-end]

---