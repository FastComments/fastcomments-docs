## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createEmailTemplateBody | CreateEmailTemplateBody | No |  |

## Response

Returns: [`Option[CreateEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_email_template200response.nim)

## Example

[inline-code-attrs-start title = 'createEmailTemplate Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let createBody = CreateEmailTemplateBody(
  name = "Weekly Newsletter",
  subject = "Your Weekly News Roundup",
  fromName = "News Team",
  fromEmail = "news@my-tenant-123.com",
  replyTo = "reply@my-tenant-123.com",
  html = "<h1>News</h1><p>Top stories this week...</p>",
  text = "News\nTop stories this week...",
  enabled = true,
  tags = @["newsletter", "weekly"],
  locale = "en-US"
)

let (response, httpResponse) = client.createEmailTemplate(tenantId = "my-tenant-123", createEmailTemplateBody = createBody)

if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]
