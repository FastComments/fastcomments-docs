## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`Option[GetTicket_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket200response.nim)

## 예제

[inline-code-attrs-start title = 'getTicket 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "", userId = "")
if response.isSome:
  let ticket = response.get()
  discard ticket
[inline-code-end]

---