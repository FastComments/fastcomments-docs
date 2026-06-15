## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'pinComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-72";
const commentId: string = "cmt_8f3a2b4c9d";
const broadcastId: string = "live_2026-06-15_21z";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload.signature";

const responseNoSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]

---