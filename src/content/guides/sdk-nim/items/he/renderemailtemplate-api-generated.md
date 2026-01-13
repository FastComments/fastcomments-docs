## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | לא |  |
| locale | string | לא |  |

## תגובה

מחזיר: [`Option[RenderEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template200response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-renderEmailTemplate'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let renderBody = RenderEmailTemplateBody(templateId: "comment-notification", subject: "New comment on your article", variables: @["John Doe", "news/global-climate"])
let (response, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = renderBody, locale = "en-US")
if response.isSome:
  let rendered = response.get()
  echo rendered
[inline-code-end]

---