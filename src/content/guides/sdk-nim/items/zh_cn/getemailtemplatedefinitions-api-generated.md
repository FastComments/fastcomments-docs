## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 响应

返回: [`Option[GetEmailTemplateDefinitions_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions200response.nim)

## 示例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if response.isSome:
  let defs = response.get()
  echo "Received email template definitions for tenant my-tenant-123"
else:
  echo "No template definitions returned; HTTP status: ", httpResponse.status
[inline-code-end]

---