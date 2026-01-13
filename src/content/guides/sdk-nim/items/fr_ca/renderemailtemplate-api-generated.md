## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | Non |  |
| locale | string | Non |  |

## Réponse

Retourne: [`Option[RenderEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let renderBody = RenderEmailTemplateBody(templateId: "comment-notification", subject: "New comment on your article", variables: @["John Doe", "news/global-climate"])
let (response, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = renderBody, locale = "en-US")
if response.isSome:
  let rendered = response.get()
  echo rendered
[inline-code-end]

---