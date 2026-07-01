## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|------------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[GetEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email")
if response.isSome:
  let tmpl = response.get()
[inline-code-end]

---