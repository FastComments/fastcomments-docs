## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| broadcastId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer pinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f2b9a";
const commentId: string = "cmt_9f8e7d6c";
const broadcastId: string = "brd_live_concert_2026-03-25";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload_signature";

const result: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]