## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text200response.nim)

## 例

[inline-code-attrs-start title = 'getCommentText の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentText(tenantId = "my-tenant-123", commentId = "cmt-456789", editKey = "", sso = "")

if response.isSome:
  let comment = response.get()
  echo "Comment text: ", $comment
else:
  echo "No comment returned"
[inline-code-end]

---