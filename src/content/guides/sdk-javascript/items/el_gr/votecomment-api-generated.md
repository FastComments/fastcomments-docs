## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| urlId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| voteBodyParams | VoteBodyParams | Ναι |  |
| sessionId | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα voteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c';
const commentId: string = 'cmt_42f3a1';
const urlId: string = 'articles/ai-trends-2026';
const broadcastId: string = 'web';
const voteBodyParams: VoteBodyParams = { vote: 1, reason: 'Insightful and on-topic' };
const sessionId: string = 'sess_6d2b4c9e';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: VoteComment200Response = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso);
[inline-code-end]

---