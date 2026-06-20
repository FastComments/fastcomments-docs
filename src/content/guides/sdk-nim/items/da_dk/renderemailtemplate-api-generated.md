## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Nej |  |
| locale | string | Nej |  |

## Svar

Returnerer: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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