## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| broadcastId | string | Sì |  |
| editKey | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2c9b';
const commentId: string = 'comment-7c3a9f2d';
const broadcastId: string = 'article-2026-03-20';
const editKey: string | undefined = 'ek_pub_abc12345';
const sso: string | undefined = 'sso_eyJhbGciOiJIUzI1Ni';

const result: DeleteCommentPublic200Response = await deleteCommentPublic(
  tenantId,
  commentId,
  broadcastId,
  editKey,
  sso
);
[inline-code-end]

---