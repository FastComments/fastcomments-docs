---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 否 |  |

## 响应

返回：[`Option[CreateEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_email_template_response.nim)

## 示例

[inline-code-attrs-start title = 'createEmailTemplate 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createEmailTemplate(tenantId = "my-tenant-123",
  createEmailTemplateBody = CreateEmailTemplateBody(
    name = "Weekly Newsletter",
    subject = "This Week on NewsSite",
    html = "<h1>Latest updates</h1><p>Read our latest article.</p>",
    fromAddress = "no-reply@newssite.com",
    isDefault = false,
    tags = @["news", "weekly"]
  )
)

if response.isSome:
  let template = response.get()
  echo "Created email template id: ", template.id
[inline-code-end]

---