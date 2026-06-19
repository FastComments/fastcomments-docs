## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| voteBodyParams | VoteBodyParams | Ja |  |
| sessionId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`VoteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'voteComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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