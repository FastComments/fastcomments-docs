## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | No |  |
| locale | string | No |  |

## Risposta

Restituisce: [`Option[RenderEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template200response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let renderBody = RenderEmailTemplateBody(templateId: "comment-notification", subject: "New comment on your article", variables: @["John Doe", "news/global-climate"])
let (response, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = renderBody, locale = "en-US")
if response.isSome:
  let rendered = response.get()
  echo rendered
[inline-code-end]

---