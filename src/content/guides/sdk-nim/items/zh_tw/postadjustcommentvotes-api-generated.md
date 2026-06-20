## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## 範例

[inline-code-attrs-start title = 'postAdjustCommentVotes 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]

---