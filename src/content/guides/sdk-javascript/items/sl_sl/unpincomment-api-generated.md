## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_org_001';
const commentId: string = 'cmt_4f2b1a8e';
const broadcastId: string = 'bcast_live_2026-06-19';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoiamRvZSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

const response: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---