## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Non |  |

## Réponse

Renvoie : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de mise à jour du modèle d\'email'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateEmailTemplateBody(
  subject: "Welcome to FastComments",
  body: "Hello \{{user_name}}, thanks for joining!",
  enabled: true,
)

let (maybeResp, httpResp) = client.updateEmailTemplate(
  tenantId = "my-tenant-123",
  id = "welcome-email",
  updateEmailTemplateBody = updateBody,
)

if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]