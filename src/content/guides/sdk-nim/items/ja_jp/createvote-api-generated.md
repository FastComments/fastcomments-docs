## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| direction | string | いいえ |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`Option[VoteComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_comment200response.nim)

## 例

[inline-code-attrs-start title = 'createVote の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createVote(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654321",
  direction = "up",
  userId = "user-42",
  anonUserId = ""
)
if response.isSome:
  let vote = response.get()
  echo "Vote recorded: ", $vote
else:
  echo "Vote not created, HTTP response: ", $httpResponse
[inline-code-end]

---