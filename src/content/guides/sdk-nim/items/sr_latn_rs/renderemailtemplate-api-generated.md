## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Ne |  |
| locale | string | Ne |  |

## Odgovor

Vraća: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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