## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | No |  |
| options | PostAdjustCommentVotesOptions | No |  |

## תגובה

מחזיר: [`Option[AdjustVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_adjust_votes_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑postAdjustCommentVotes'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (adjustRespOpt, httpResp) = client.postAdjustCommentVotes(
  tenantId = "my-tenant-123",
  commentId = "cmt-789",
  adjustCommentVotesParams = AdjustCommentVotesParams(),
  options = PostAdjustCommentVotesOptions()
)

if adjustRespOpt.isSome:
  let adjustResp = adjustRespOpt.get()
[inline-code-end]