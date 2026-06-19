## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nej |  |

## Respons

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'unLockComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3d2b';
const commentId: string = 'cmt_5e8a1d';
const broadcastId: string = 'bcast_4f2b7a';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1aWQiOiJ1c2VyX2QxMjMifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const result: APIEmptyResponse = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---