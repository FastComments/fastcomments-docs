## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| createEmailTemplateBody | CreateEmailTemplateBody | Non |  |

## Réponse

Retourne : [`Option[CreateEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_email_template_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateEmailTemplateBody(
  name: "Welcome Email",
  subject: "Welcome to FastComments",
  htmlBody: "<p>Hello, thank you for joining us!</p>",
  isActive: true,
  tags: @[]
)

let (optResp, httpResp) = client.createEmailTemplate(
  tenantId = "my-tenant-123",
  createEmailTemplateBody = createBody
)

if optResp.isSome:
  let emailTemplate = optResp.get()
[inline-code-end]

---