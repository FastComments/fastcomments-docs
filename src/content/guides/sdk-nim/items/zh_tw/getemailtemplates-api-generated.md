## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 回傳

回傳：[`Option[GetEmailTemplates_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates200response.nim)

## 範例

[inline-code-attrs-start title = 'getEmailTemplates 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let templates = response.get()
  echo templates
else:
  echo "No templates returned"
[inline-code-end]