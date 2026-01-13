## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetVotesForUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user200response.nim)

## 例

[inline-code-attrs-start title = 'getVotesForUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/economy-update-2026",
  userId = "user-789",
  anonUserId = ""
)

if response.isSome:
  let votes = response.get()
  discard votes
[inline-code-end]

---