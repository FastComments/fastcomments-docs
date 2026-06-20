## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## レスポンス

戻り値: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## 例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if response.isSome:
  let definitions = response.get()
  echo "Email template definitions for my-tenant-123: ", definitions
else:
  echo "Failed to retrieve templates, HTTP status: ", httpResponse.status
[inline-code-end]

---