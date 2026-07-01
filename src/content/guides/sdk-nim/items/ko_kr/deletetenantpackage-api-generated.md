## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteTenantPackage 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponse, httpResponse) = client.deleteTenantPackage(tenantId = "my-tenant-123", id = "pkg-456def")
if apiResponse.isSome:
  let emptyResponse = apiResponse.get()
[inline-code-end]