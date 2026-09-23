## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| voteId | string | Sí |  |
| urlId | string | Sí |  |
| broadcastId | string | Sí |  |
| editKey | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteDeleteResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteCommentVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const voteId: string = "vote_555";
const urlId: string = "url_abcde";
const broadcastId: string = "brd_2023";
const editKey: string = "edit_abc123"; // opcional
const ssoToken: string = "sso_token_xyz"; // opcional

// Llamada con solo los parámetros obligatorios y uno opcional
const deleteResult: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey
);

// Llamada con ambos parámetros opcionales
const deleteResultWithSSO: VoteDeleteResponse = await deleteCommentVote(
  tenantId,
  commentId,
  voteId,
  urlId,
  broadcastId,
  editKey,
  ssoToken
);
[inline-code-end]

---