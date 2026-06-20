---
## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[GetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_text_response.nim)

## 範例

[inline-code-attrs-start title = 'getModerationCommentText 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerationCommentText(commentId = "comment-9f8b7a6c", sso = "")
if response.isSome:
  let commentData = response.get()
  echo "Moderation comment text retrieved"
else:
  echo "No moderation comment text available"
[inline-code-end]

---