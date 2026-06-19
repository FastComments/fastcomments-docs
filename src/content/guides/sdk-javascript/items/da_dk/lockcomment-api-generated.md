## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'lockComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media-214';
const commentId: string = 'cmt_4f3b9a2d';
const broadcastId: string = 'live-987654321';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1NzgiLCJuYW1lIjoiSmFuZSBEb2UifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const lockedWithSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId, sso);
const lockedWithoutSso: APIEmptyResponse = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]

---