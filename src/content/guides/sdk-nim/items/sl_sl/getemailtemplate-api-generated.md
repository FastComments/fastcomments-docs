## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vrne: [`Option[GetEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email-01")
if response.isSome:
  let template = response.get()
  discard template
[inline-code-end]