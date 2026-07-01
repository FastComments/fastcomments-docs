## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |

## 응답

반환: [`Option[GetEmailTemplateDefinitionsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_definitions_response.nim)

## 예시

[inline-code-attrs-start title = 'getEmailTemplateDefinitions 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.getEmailTemplateDefinitions(tenantId = "my-tenant-123")
if responseOpt.isSome:
  let definitions = responseOpt.get()
  echo definitions
[inline-code-end]