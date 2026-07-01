## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|---------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Так |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |

## Відповідь

Повертає: [`UnBlockUserFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockUserFromCommentResponse.ts)

## Приклад

[inline-code-attrs-start title = 'unBlockUserFromComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    // anonUserId пропущено
  );

  console.log(result);
}
demoUnblock();
[inline-code-end]

---