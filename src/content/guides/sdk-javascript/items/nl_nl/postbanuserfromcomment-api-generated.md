## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| commentId | string | Ja |  |
| banEmail | boolean | Nee |  |
| banEmailDomain | boolean | Nee |  |
| banIP | boolean | Nee |  |
| deleteAllUsersComments | boolean | Nee |  |
| bannedUntil | string | Nee |  |
| isShadowBan | boolean | Nee |  |
| updateId | string | Nee |  |
| banReason | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserFromComment Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // e-mail verbannen
  false,       // e-maildomein verbannen
  true,        // IP-adres verbannen
  true,        // alle reacties van gebruiker verwijderen
  bannedUntil,
  false,       // schaduwverbanning
  updateId,
  banReason,
  sso
);
[inline-code-end]