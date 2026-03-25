## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9d2a3b';
const commentId: string = 'comment_842b9c1f';
const broadcastId: string = 'bcast_frontpage_202603';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const result: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---