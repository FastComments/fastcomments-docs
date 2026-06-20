## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| setCommentTextParams | SetCommentTextParams | 否 |  |
| sso | string | 否 |  |

## 回應

回傳：[`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## 範例

[inline-code-attrs-start title = 'postSetCommentText 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentText(commentId = "comment-4821",
  setCommentTextParams = SetCommentTextParams(text = "Updated comment to clarify the main point and fix a typo."),
  sso = "sso-user-8f3b9c")

if response.isSome:
  let setCommentResp = response.get()
  echo "Received SetCommentTextResponse"
[inline-code-end]

---