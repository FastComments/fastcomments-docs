## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
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
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // banEmail — forbyd e-mailadressen
  false,       // banEmailDomain — forbyd e-maildomænet
  true,        // banIP — forbyd IP-adressen
  true,        // deleteAllUsersComments — slet alle brugerens kommentarer
  bannedUntil,
  false,       // isShadowBan — skyggebanning (brugeren opdager det ikke)
  updateId,
  banReason,
  sso
);
[inline-code-end]