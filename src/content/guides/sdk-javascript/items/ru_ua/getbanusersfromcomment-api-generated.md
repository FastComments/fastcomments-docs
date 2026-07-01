## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| commentId | string | Так |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getBanUsersFromComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Виклик з усіма параметрами
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Виклик лише з обов’язковим параметром
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]