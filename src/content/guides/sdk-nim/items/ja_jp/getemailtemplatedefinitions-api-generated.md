## パラメータ

| Name | Type | Required | 説明 |
|------|------|----------|------|
| tenantId | string | Yes |  |

## レスポンス

戻り値: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## 例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if responseOpt.isSome:
  let definitions = responseOpt.get()
  echo definitions
[inline-code-end]