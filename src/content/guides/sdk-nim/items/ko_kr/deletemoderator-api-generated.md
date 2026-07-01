## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| sendEmail | string = "" | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteModerator 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteModerator(
  tenantId = "my-tenant-123",
  id = "mod-789",
  sendEmail = "admin@mydomain.com",
)

if apiResp.isSome:
  let empty = apiResp.get()
  echo "Moderator removed"
[inline-code-end]