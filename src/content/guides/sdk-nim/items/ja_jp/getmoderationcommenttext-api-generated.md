## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| commentId | string | 必須 |  |
| sso | string | 任意 |  |

## レスポンス

戻り値: [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## 例

[inline-code-attrs-start title = 'getModerationCommentText の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerationCommentText(commentId = "comment-9f8b7a6c", sso = "")
if response.isSome:
  let commentData = response.get()
  echo "Moderation comment text retrieved"
else:
  echo "No moderation comment text available"
[inline-code-end]

---