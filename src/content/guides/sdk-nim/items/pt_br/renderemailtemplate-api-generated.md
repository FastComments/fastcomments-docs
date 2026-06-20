## Parâmetros

| Name | Type | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Não |  |
| locale | string | Não |  |

## Resposta

Retorna: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.renderEmailTemplate(
  tenantId = "my-tenant-123",
  renderEmailTemplateBody = RenderEmailTemplateBody(),
  locale = "en-US"
)

if response.isSome:
  let rendered = response.get()
  echo rendered
[inline-code-end]

---