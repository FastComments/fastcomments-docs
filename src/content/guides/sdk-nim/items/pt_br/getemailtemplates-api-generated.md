## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| skip | float64 | Não |  |

## Resposta

Retorna: [`Option[GetEmailTemplatesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEmailTemplates'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let templates = response.get()
  echo templates
else:
  echo "No email templates available"
[inline-code-end]