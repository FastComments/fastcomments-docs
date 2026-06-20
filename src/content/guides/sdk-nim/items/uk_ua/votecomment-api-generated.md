---
## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| urlId | string | Так |  |
| broadcastId | string | Ні |  |
| voteBodyParams | VoteBodyParams | Ні |  |
| sessionId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Приклад

[inline-code-attrs-start title = 'voteComment Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.voteComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654321",
  urlId = "news/article-2026-inflation",
  broadcastId = "",
  voteBodyParams = VoteBodyParams(),
  sessionId = "",
  sso = ""
)

if response.isSome:
  let voteResp = response.get()
  discard voteResp
else:
  discard httpResponse
[inline-code-end]

---