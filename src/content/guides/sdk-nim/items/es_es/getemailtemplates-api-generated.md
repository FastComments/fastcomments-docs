## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | float64 | No |  |

## Respuesta

Devuelve: [`Option[GetEmailTemplatesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEmailTemplates'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let templates = response.get()
  echo templates
else:
  echo "No email templates available"
[inline-code-end]

---