## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIGetCommentTextResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const commentId: string = "comment-987654321";
const editKey: string = "edit-key-abc123";
const sso: string = "sso-token-xyz789";

const commentWithEdit: PublicAPIGetCommentTextResponse = await getCommentText(
  tenantId,
  commentId,
  editKey,
  sso
);

const commentBasic: PublicAPIGetCommentTextResponse = await getCommentText(
  tenantId,
  commentId
);
[inline-code-end]