## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Non |  |
| locale | string | Non |  |

## Réponse

Renvoie: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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