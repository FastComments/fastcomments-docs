## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | כן |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postAdjustCommentVotes(commentId = "cmt-987654", adjustCommentVotesParams = nil, sso = "sso-token-abc123")
if response.isSome:
  let adjusted = response.get()
  discard adjusted
[inline-code-end]

---