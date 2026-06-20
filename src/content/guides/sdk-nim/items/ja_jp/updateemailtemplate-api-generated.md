## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| updateEmailTemplateBody | UpdateEmailTemplateBody | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'updateEmailTemplate の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateEmailTemplateBody(
  subject = "Welcome to Newsly",
  html = "<p>Thanks for joining Newsly! Visit https://newsly.example to get started.</p>",
  fromAddress = "no-reply@newsly.example",
  fromName = "Newsly Team",
  enabled = true
)
let (response, httpResponse) = client.updateEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email", updateEmailTemplateBody = updateBody)
if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse.statusCode
[inline-code-end]

---