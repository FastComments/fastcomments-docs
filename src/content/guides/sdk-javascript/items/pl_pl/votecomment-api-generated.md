## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| urlId | string | Tak |  |
| broadcastId | string | Tak |  |
| voteBodyParams | VoteBodyParams | Tak |  |
| sessionId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia voteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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