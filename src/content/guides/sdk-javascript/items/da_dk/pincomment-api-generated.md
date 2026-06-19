## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'pinComment-eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82b1f9';
const commentId: string = 'cmt_9f8e7d6a';
const broadcastId: string = 'live_brdcst_2026_06_19';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const responseWithoutSSO: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---