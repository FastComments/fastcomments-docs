## פרמטרים

| שם | טיפוס | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | No |  |
| locale | string = "" | No |  |

## תשובה

מחזיר: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = RenderEmailTemplateBody()
let (responseOpt, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = body, locale = "en-US")
if responseOpt.isSome:
  let response = responseOpt.get()
  discard response
discard httpResponse
[inline-code-end]