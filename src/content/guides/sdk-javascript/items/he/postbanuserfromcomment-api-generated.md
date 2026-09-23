## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| banEmail | boolean | לא |  |
| banEmailDomain | boolean | לא |  |
| banIP | boolean | לא |  |
| deleteAllUsersComments | boolean | לא |  |
| bannedUntil | string | לא |  |
| isShadowBan | boolean | לא |  |
| updateId | string | לא |  |
| banReason | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (הושמט)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (הושמט)
    "Harassment",             // banReason
    undefined                 // sso (הושמט)
  );

  console.log(result);
}
[inline-code-end]