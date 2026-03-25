## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'unBlockCommentPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42e8a1';
const commentId: string = 'cmt_9b3f2d';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'abusive_language',
  blockedByModeratorId: 'mod_17',
  note: 'Targeted harassment; review complete',
  unblockRequestedAt: new Date().toISOString()
};
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
const result: UnBlockCommentPublic200Response = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]