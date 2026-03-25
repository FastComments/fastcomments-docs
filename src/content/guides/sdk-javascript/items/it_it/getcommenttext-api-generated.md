## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| editKey | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentText200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const commentId: string = 'cmt_7890b';
const editKey: string = 'edit_4f2d9b7c';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';

const result: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]