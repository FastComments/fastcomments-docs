## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[APIGetCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comment_response.nim)

## 例

[inline-code-attrs-start title = 'getComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-7890")
if response.isSome:
  let comment = response.get()
  discard comment
else:
  echo "Comment not found"
[inline-code-end]

---