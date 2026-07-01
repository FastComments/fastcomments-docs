## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 回應

返回: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## 範例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if responseOpt.isSome:
  let definitions = responseOpt.get()
  echo definitions
[inline-code-end]

---