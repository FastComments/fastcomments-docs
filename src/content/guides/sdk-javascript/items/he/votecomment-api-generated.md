## פרמטרים

| שם | Type | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| urlId | string | כן |  |
| broadcastId | string | כן |  |
| voteBodyParams | VoteBodyParams | כן |  |
| sessionId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת voteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b2f9c';
const commentId: string = 'cmt_4a9e2d';
const urlId: string = 'articles/2026/new-features';
const broadcastId: string = 'brd_1f3a9b';
const voteBodyParams: VoteBodyParams = { vote: 'up' };
const sessionId: string = 'sess_ab12cd34';
const voteResponse: VoteResponse = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId);
[inline-code-end]

---