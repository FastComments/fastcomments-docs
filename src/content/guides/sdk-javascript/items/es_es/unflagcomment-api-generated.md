## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`UnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnFlagCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const commentId: string = "cmt_123456";
const userId: string = "usr_98765";

const result: UnFlagCommentResponse = await unFlagComment(tenantId, commentId, userId);

const anonCommentId: string = "cmt_123457";
const anonUserId: string = "anon_abc123";

const anonResult: UnFlagCommentResponse = await unFlagComment(tenantId, anonCommentId, undefined, anonUserId);
[inline-code-end]