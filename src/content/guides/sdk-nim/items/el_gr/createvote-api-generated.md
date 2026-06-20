## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| direction | string | Όχι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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