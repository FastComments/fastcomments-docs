## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| banEmail | boolean | No |  |
| banEmailDomain | boolean | No |  |
| banIP | boolean | No |  |
| deleteAllUsersComments | boolean | No |  |
| bannedUntil | string | No |  |
| isShadowBan | boolean | No |  |
| updateId | string | No |  |
| banReason | string | No |  |
| sso | string | No |  |

## Одговор

Враћа: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // забрана е‑поште
    undefined,                // banEmailDomain (изостављено)
    true,                     // забрана IP‑а
    false,                    // брисање свих коментара корисника
    "2025-01-01T00:00:00Z",   // забрањено до
    true,                     // је сенчана забрана
    undefined,                // updateId (изостављено)
    "Harassment",             // разлог забране
    undefined                 // sso (изостављено)
  );

  console.log(result);
}
[inline-code-end]