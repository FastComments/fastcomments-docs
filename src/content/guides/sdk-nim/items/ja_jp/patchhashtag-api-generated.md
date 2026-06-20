## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tag | string | いいえ |  |
| tenantId | string | はい |  |
| updateHashTagBody | UpdateHashTagBody | いいえ |  |

## レスポンス

戻り値: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## 例

[inline-code-attrs-start title = 'patchHashTag の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---