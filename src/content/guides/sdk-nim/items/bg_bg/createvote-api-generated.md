## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| direction | string | Не |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |

## Отговор

Връща: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Пример

[inline-code-attrs-start title = 'createVote Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  direction = "up",
  userId = "user-42",
  anonUserId = ""
)
if response.isSome:
  let vote = response.get()
  echo "Vote created:", vote
else:
  echo "No vote returned"
[inline-code-end]

---