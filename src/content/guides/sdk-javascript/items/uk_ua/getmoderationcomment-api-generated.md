## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // Повний набір параметрів
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // Мінімальний виклик, що використовує лише обов'язковий аргумент
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // Використовуйте результати за потребою...
}
[inline-code-end]