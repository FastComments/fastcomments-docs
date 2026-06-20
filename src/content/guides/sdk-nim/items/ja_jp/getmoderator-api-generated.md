---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## 例

[inline-code-attrs-start title = 'getModerator の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-456")
if response.isSome:
  let moderator = response.get()
  echo moderator
else:
  echo "Moderator not found, HTTP status: ", $httpResponse.status
[inline-code-end]

---