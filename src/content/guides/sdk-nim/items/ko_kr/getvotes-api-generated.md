---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |

## 응답

반환: [`Option[GetVotes_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes200response.nim)

## 예제

[inline-code-attrs-start title = 'getVotes 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/2026/major-update")
if response.isSome:
  let votes = response.get()
  discard votes
else:
  discard httpResponse
[inline-code-end]

---