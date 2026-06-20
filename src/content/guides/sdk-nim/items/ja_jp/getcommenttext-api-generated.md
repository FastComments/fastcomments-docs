## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## 例

[inline-code-attrs-start title = 'getCommentText の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentText(tenantId = "my-tenant-123", commentId = "cmt-987654321", editKey = "", sso = "")

if response.isSome:
  let commentTextResp = response.get()
  echo commentTextResp
else:
  echo "No comment text returned"
[inline-code-end]

---