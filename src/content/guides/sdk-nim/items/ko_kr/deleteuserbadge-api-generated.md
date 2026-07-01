## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 응답

반환: [`Option[APIEmptySuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_success_response.nim)

## 예제

[inline-code-attrs-start title = 'deleteUserBadge 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponse, httpResponse) = client.deleteUserBadge(tenantId = "my-tenant-123", id = "badge-456")
if apiResponse.isSome:
  let success = apiResponse.get()
[inline-code-end]