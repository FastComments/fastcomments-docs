## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
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

## Primjer

[inline-code-attrs-start title = 'postBanUserFromComment Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function banUserExample(): Promise<void> {
  const result: BanUserFromCommentResult = await postBanUserFromComment(
    "tenant_42",
    "comment_1001",
    true,                     // zabraniti email
    undefined,                // banEmailDomain (izostavljeno)
    true,                     // zabraniti IP
    false,                    // obrisati sve komentare korisnika
    "2025-01-01T00:00:00Z",   // zabranjeno do
    true,                     // je li sjenovna zabrana
    undefined,                // updateId (izostavljeno)
    "Harassment",             // razlog zabrane
    undefined                 // sso (izostavljeno)
  );

  console.log(result);
}
[inline-code-end]