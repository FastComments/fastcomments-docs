## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |

## Відповідь

Повертає: [`FlagCommentResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_2023";
  const commentId: string = "comment_5678";
  const userId: string = "user_1234";
  const anonUserId: string = "anon_4321";

  const result: FlagCommentResponse1 = await flagComment(tenantId, commentId, userId);
  const anonResult: FlagCommentResponse1 = await flagComment(tenantId, commentId, undefined, anonUserId);
})();
[inline-code-end]