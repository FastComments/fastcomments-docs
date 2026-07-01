## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| redirectURL | string = "" | 아니오 |  |

## 응답

반환값: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'sendLoginLink 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.sendLoginLink(
  tenantId = "my-tenant-123",
  id = "user-456",
  redirectURL = "https://myapp.example.com/login-success"
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]