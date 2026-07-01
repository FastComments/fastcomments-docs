## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| id | string | Да |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Да |  |
| userId | string | Нет |  |
| anonUserId | string | Нет |  |

## Ответ

Возвращает: [`UnBlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockUserFromCommentResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnblock() {
  const tenantId: string = "acme-corp-tenant";
  const commentId: string = "cmt_9f8b7a6d";

  const params: UnBlockFromCommentParams = {
    reason: "User resolved the issue",
    notifyUser: true
  };

  const userId: string = "usr_12345";

  const result: UnBlockUserFromCommentResponse = await unBlockUserFromComment(
    tenantId,
    commentId,
    params,
    userId
    // anonUserId опущен
  );

  console.log(result);
}
demoUnblock();
[inline-code-end]