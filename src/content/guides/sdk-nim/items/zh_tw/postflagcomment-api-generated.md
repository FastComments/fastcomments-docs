## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| options | PostFlagCommentOptions | 否 |  |

## 回應

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'postFlagComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = PostFlagCommentOptions()
let (response, httpResponse) = client.postFlagComment(
  tenantId = "my-tenant-123",
  commentId = "comment-987654",
  options = opts,
)
if response.isSome:
  let result = response.get()
[inline-code-end]

---