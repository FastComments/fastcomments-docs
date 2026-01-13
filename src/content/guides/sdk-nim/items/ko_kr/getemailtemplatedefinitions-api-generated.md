---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`Option[GetEmailTemplateDefinitions_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions200response.nim)

## 예제

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if response.isSome:
  let defs = response.get()
  echo "Received email template definitions for tenant my-tenant-123"
else:
  echo "No template definitions returned; HTTP status: ", httpResponse.status
[inline-code-end]

---