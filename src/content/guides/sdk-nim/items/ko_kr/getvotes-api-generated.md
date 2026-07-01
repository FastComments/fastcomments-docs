## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## 응답

반환: [`Option[GetVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_response.nim)

## 예시

[inline-code-attrs-start title = 'getVotes 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optVotes, httpResp) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/article-42")
if optVotes.isSome:
  let votes = optVotes.get()
  discard votes
[inline-code-end]