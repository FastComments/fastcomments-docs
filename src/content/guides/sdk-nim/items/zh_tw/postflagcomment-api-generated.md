---
## 參數

| 名稱 | 類型 | 必要 | 描述 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'postFlagComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postFlagComment(commentId = "comment-742", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Comment flagged successfully"
else:
  echo "Failed to flag comment"
[inline-code-end]

---