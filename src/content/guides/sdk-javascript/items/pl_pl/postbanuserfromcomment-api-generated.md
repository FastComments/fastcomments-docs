## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| banEmail | boolean | Nie |  |
| banEmailDomain | boolean | Nie |  |
| banIP | boolean | Nie |  |
| deleteAllUsersComments | boolean | Nie |  |
| bannedUntil | string | Nie |  |
| isShadowBan | boolean | Nie |  |
| updateId | string | Nie |  |
| banReason | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // banEmail
    undefined,                // banEmailDomain (pominięto)
    true,                     // banIP
    false,                    // deleteAllUsersComments
    "2025-01-01T00:00:00Z",   // bannedUntil
    true,                     // isShadowBan
    undefined,                // updateId (pominięto)
    "Harassment",             // banReason
    undefined                 // sso (pominięto)
  );

  console.log(result);
}
[inline-code-end]