## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Nee |  |
| locale | string | Nee |  |

## Antwoord

Retourneert: [`Option[RenderEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'renderEmailTemplate Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let renderBody = RenderEmailTemplateBody(templateId: "comment-notification", subject: "New comment on your article", variables: @["John Doe", "news/global-climate"])
let (response, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = renderBody, locale = "en-US")
if response.isSome:
  let rendered = response.get()
  echo rendered
[inline-code-end]

---