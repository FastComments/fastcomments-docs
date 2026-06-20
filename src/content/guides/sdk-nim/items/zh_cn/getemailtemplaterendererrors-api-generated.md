---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| skip | float64 | 否 |  |

## 响应

返回: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## 示例

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateRenderErrors(tenantId = "my-tenant-123", id = "", skip = 0.0)
if response.isSome:
  let templateErrors = response.get()
  discard templateErrors
else:
  discard httpResponse
[inline-code-end]

---