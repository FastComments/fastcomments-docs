## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| approved | bool | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## 範例

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]

---