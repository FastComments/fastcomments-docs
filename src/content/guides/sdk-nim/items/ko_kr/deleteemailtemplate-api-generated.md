## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteEmailTemplate 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email")
if respOpt.isSome:
  let emptyResp = respOpt.get()
  echo "Email template deleted"
else:
  echo "Failed to delete email template"
[inline-code-end]