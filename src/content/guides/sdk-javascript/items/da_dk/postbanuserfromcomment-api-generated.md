## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| banEmail | boolean | Nej |  |
| banEmailDomain | boolean | Nej |  |
| banIP | boolean | Nej |  |
| deleteAllUsersComments | boolean | Nej |  |
| bannedUntil | string | Nej |  |
| isShadowBan | boolean | Nej |  |
| updateId | string | Nej |  |
| banReason | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Eksempel

[inline-code-attrs-start title = 'postBanUserFromComment Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (udeladt)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (udeladt)
    "Harassment",             // banReason
    undefined                 // sso (udeladt)
  );

  console.log(result);
}
[inline-code-end]