## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updatableCommentParams | UpdatableCommentParams | Sí |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-prod-01';
const id: string = 'cmt-000127';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Thanks — I updated the steps to include the missing config flag.',
  isHidden: false
};
const contextUserId: string = 'moderator_77';
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: APIEmptyResponse = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---