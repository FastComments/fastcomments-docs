## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updatableCommentParams | UpdatableCommentParams | Sí |  |
| contextUserId | string | No |  |
| doSpamCheck | boolean | No |  |
| isLive | boolean | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3c1b2a';
const commentId: string = 'cmt_8d9f2a4b';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Updating this comment to clarify the feature behavior and include a timestamp.',
  metadata: { category: 'support', editedReason: 'clarify instructions' },
  visible: true
};
const contextUserId: string = 'user_42';
const doSpamCheck: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, commentId, updatableCommentParams, contextUserId, doSpamCheck);
[inline-code-end]

---