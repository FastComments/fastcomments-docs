## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'deleteEmailTemplate 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplate(
  tenantId = "my-tenant-123",
  id = "welcome-email-template-001"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
  echo "Email template deleted successfully"
else:
  echo "No response body"
[inline-code-end]

---