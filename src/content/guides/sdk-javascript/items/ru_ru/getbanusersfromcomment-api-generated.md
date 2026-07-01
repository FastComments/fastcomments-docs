## Параметры

| Название | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| tenantId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetBanUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBanUsersFromCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getBanUsersFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetBanUsers() {
  const commentId: string = "cmt_5f8e3a9b2d";
  const tenantId: string = "tenant_42";
  const sso: string = "sso_token_abc123";

  // Вызов со всеми параметрами
  const fullResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId, tenantId, sso);
  console.log(fullResult);

  // Вызов только с обязательным параметром
  const minimalResult: GetBanUsersFromCommentResponse = await getBanUsersFromComment(commentId);
  console.log(minimalResult);
}

demoGetBanUsers();
[inline-code-end]