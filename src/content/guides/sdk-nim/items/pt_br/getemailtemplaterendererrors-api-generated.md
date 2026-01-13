## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| skip | float64 | Não |  |

## Resposta

Retorna: [`Option[GetEmailTemplateRenderErrors_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getEmailTemplateRenderErrors'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateRenderErrors(tenantId = "my-tenant-123", id = "welcome-email-template-001", skip = 0.0)
if response.isSome:
  let result = response.get()
  echo "Render errors:", result
else:
  echo "No render errors or request failed. HTTP status:", httpResponse.status
[inline-code-end]

---