## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|-----------|
| tenantId | string | Ναι |  |
| id | string | Όχι |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'updateEmailTemplate Παράδειγμα'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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