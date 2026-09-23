## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| banEmail | boolean | Не |  |
| banEmailDomain | boolean | Не |  |
| banIP | boolean | Не |  |
| deleteAllUsersComments | boolean | Не |  |
| bannedUntil | string | Не |  |
| isShadowBan | boolean | Не |  |
| updateId | string | Не |  |
| banReason | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (пропуснато)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (пропуснато)
    "Harassment",             // banReason
    undefined                 // sso (пропуснато)
  );

  console.log(result);
}
[inline-code-end]