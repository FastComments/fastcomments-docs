## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| skip | float64 | No |  |

## Response

מחזיר: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## Example

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors דוגמה'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getEmailTemplateRenderErrors(
  tenantId = "my-tenant-123",
  id = "welcome-template",
  skip = 0.0
)

if optResp.isSome:
  let resp = optResp.get()
  # השתמש בתגובה לפי הצורך
else:
  # טפל במצב שבו אין תגובה
  discard
[inline-code-end]