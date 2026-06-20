## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |

## レスポンス

返却値: [`Option[GetVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_response.nim)

## 例

[inline-code-attrs-start title = 'getVotes の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/breaking-article-456")
if response.isSome:
  let votesResp = response.get()
  echo "Received votes response:", $votesResp
else:
  echo "No votes returned, HTTP response:", $httpResponse
[inline-code-end]

---