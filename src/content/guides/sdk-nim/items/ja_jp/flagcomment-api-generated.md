---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| userId | string | いいえ |  |
| anonUserId | string | いいえ |  |

## レスポンス

戻り値: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## 例

[inline-code-attrs-start title = 'flagCommentの例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-98765", userId = "user-8342", anonUserId = "")
if response.isSome:
  let flagged = response.get()
  echo "Flagged comment response: ", flagged
else:
  echo "Flag comment failed: ", httpResponse
[inline-code-end]

---