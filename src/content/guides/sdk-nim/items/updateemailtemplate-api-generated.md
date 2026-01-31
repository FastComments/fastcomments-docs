## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'updateEmailTemplate Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateEmailTemplateBody(
  subject: "Welcome to Newsly",
  html: "<p>Hi {name}, welcome to Newsly. Visit <a href=\"{link}\">your dashboard</a>.</p>",
  enabled: true,
  fromAddress: "no-reply@newsly.com",
  recipients: @["subscribers@newsly.com"]
)

let (response, httpResponse) = client.updateEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email-template-42", updateEmailTemplateBody = body)

if response.isSome:
  let updated = response.get()
  echo updated
[inline-code-end]
