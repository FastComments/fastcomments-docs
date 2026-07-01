## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| commentId | string | Так |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getModerationCommentText Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // Виклик лише з обов'язковим параметром
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // Виклик з необов'язковими параметрами
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]