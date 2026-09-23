## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| banEmail | boolean | Ні |  |
| banEmailDomain | boolean | Ні |  |
| banIP | boolean | Ні |  |
| deleteAllUsersComments | boolean | Ні |  |
| bannedUntil | string | Ні |  |
| isShadowBan | boolean | Ні |  |
| updateId | string | Ні |  |
| banReason | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Приклад

[inline-code-attrs-start title = 'postBanUserFromComment Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (пропущено)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (пропущено)
    "Harassment",             // banReason
    undefined                 // sso (пропущено)
  );

  console.log(result);
}
[inline-code-end]

---