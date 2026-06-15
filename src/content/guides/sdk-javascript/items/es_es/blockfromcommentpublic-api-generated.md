## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|------------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de blockFromCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6b3f9a2d';
const commentId: string = 'cmt_8f4b12a9';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'Repeated promotional links',
  durationMinutes: 60 * 24 * 30, // 30 días
  escalateToModeration: true
};
const sso: string = 'sso_token_3fH7kLw';

const result: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]

---