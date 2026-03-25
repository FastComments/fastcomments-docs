## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'unPinComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9d2a3b';
const commentId: string = 'comment_842b9c1f';
const broadcastId: string = 'bcast_frontpage_202603';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const result: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---