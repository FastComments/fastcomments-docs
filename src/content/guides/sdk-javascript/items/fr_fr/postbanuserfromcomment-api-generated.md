## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| banEmail | boolean | Non |  |
| banEmailDomain | boolean | Non |  |
| banIP | boolean | Non |  |
| deleteAllUsersComments | boolean | Non |  |
| bannedUntil | string | Non |  |
| isShadowBan | boolean | Non |  |
| updateId | string | Non |  |
| banReason | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (omise)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (omise)
    "Harassment",             // banReason
    undefined                 // sso (omise)
  );

  console.log(result);
}
[inline-code-end]