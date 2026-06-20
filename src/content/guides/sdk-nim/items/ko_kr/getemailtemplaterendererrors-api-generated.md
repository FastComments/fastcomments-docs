## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| skip | float64 | 아니요 |  |

## 응답

반환: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateRenderErrors(tenantId = "my-tenant-123", id = "", skip = 0.0)
if response.isSome:
  let templateErrors = response.get()
  discard templateErrors
else:
  discard httpResponse
[inline-code-end]

---