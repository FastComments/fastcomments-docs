## Parametri

| Ime | Tip | Obavezno | Opis |
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

## Odgovor

Vraća: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Primer

[inline-code-attrs-start title = 'postBanUserFromComment Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (izostavljeno)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (izostavljeno)
    "Harassment",             // banReason
    undefined                 // sso (izostavljeno)
  );

  console.log(result);
}
[inline-code-end]