## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | No |  |
| locale | string = "" | No |  |

## Odgovor

Vraća: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## Primer

[inline-code-attrs-start title = 'renderEmailTemplate Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = RenderEmailTemplateBody()
let (responseOpt, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = body, locale = "en-US")
if responseOpt.isSome:
  let response = responseOpt.get()
  discard response
discard httpResponse
[inline-code-end]