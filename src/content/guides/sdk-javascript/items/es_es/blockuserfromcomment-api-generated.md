## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| blockFromCommentParams | BlockFromCommentParams | Sí |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'comment_7f3b2a9c';
const blockFromCommentParams: BlockFromCommentParams = {
  reason: 'Repeated abusive language and targeted harassment',
  durationDays: 90,
  preventReposting: true
};
const userId: string = 'user_12345';
const anonUserId: string = 'anon_98765';

const result: BlockFromCommentPublic200Response = await blockUserFromComment(
  tenantId,
  id,
  blockFromCommentParams,
  userId,
  anonUserId
);
[inline-code-end]