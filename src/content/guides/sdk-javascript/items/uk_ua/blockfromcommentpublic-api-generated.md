## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Приклад

[inline-code-attrs-start title = 'blockFromCommentPublic Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6b3f9a2d';
const commentId: string = 'cmt_8f4b12a9';
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: 'Repeated promotional links',
  durationMinutes: 60 * 24 * 30, // 30 днів
  escalateToModeration: true
};
const sso: string = 'sso_token_3fH7kLw';

const result: BlockFromCommentPublic200Response = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]