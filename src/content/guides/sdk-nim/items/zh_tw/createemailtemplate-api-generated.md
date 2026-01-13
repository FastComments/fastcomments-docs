## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| createEmailTemplateBody | CreateEmailTemplateBody | 否 |  |

## 回應

回傳: [`Option[CreateEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_email_template200response.nim)

## 範例

[inline-code-attrs-start title = 'createEmailTemplate 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createEmailTemplate(tenantId = "my-tenant-123", createEmailTemplateBody = CreateEmailTemplateBody(name = "Weekly Newsletter", subject = "Weekly updates from OurSite", fromName = "OurSite Team", fromEmail = "newsletter@oursite.com", bodyHtml = "<h1>Highlights</h1><p>Top stories this week...</p>", enabled = true, tags = @["newsletter", "weekly"]))
if response.isSome:
  let template = response.get()
  discard template
[inline-code-end]

---