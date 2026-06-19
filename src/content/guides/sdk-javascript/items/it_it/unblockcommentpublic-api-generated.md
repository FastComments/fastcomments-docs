## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di unBlockCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-72';
const commentId: string = 'cmt_5f9b3a2d';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {};
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload.sig';
const result: UnblockSuccess = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]