## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`UnBlockCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublicResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unBlockCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const commentId: string = "cmt-20230915-001";
const unblockParams: PublicBlockFromCommentParams = {
  reason: "User appealed and was cleared",
  unblockExpiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString()
};
const ssoToken: string = "sso-3d9f8a7b";

const result: UnBlockCommentPublicResponse = await unBlockCommentPublic(
  tenantId,
  commentId,
  unblockParams,
  ssoToken
);
[inline-code-end]

---