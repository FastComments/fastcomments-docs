## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| setCommentTextParams | SetCommentTextParams | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## 示例

[inline-code-attrs-start title = 'postSetCommentText 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentText(commentId = "comment-4821",
  setCommentTextParams = SetCommentTextParams(text = "Updated comment to clarify the main point and fix a typo."),
  sso = "sso-user-8f3b9c")

if response.isSome:
  let setCommentResp = response.get()
  echo "Received SetCommentTextResponse"
[inline-code-end]

---