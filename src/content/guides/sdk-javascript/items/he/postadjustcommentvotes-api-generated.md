## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | כן |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AdjustVotesResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'postAdjustCommentVotes דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_987654321";
const adjustParams: AdjustCommentVotesParams = { voteDelta: 1 };
const broadcastId: string = "brd_112233";
const sso: string = "sso-token-xyz";

const result: AdjustVotesResponse = await postAdjustCommentVotes(
  tenantId,
  commentId,
  adjustParams,
  broadcastId,
  sso
);
[inline-code-end]