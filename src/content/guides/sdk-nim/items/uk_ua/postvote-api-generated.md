---
## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostVoteOptions | No |  |

## Відповідь

Повертає: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Приклад

[inline-code-attrs-start title = 'postVote Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteOpt, httpResp) = client.postVote(
  tenantId = "my-tenant-123",
  commentId = "comment-789",
  options = default(PostVoteOptions)
)

if voteOpt.isSome:
  let vote = voteOpt.get()
[inline-code-end]

---