## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
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

## Απόκριση

Επιστρέφει: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'postBanUserFromComment Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (παραλείπεται)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (παραλείπεται)
    "Harassment",             // banReason
    undefined                 // sso (παραλείπεται)
  );

  console.log(result);
}
[inline-code-end]