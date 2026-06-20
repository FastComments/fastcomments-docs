## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Da |  |
| banEmail | boolean | Ne |  |
| banEmailDomain | boolean | Ne |  |
| banIP | boolean | Ne |  |
| deleteAllUsersComments | boolean | Ne |  |
| bannedUntil | string | Ne |  |
| isShadowBan | boolean | Ne |  |
| updateId | string | Ne |  |
| banReason | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Primer

[inline-code-attrs-start title = 'Primer postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // banEmail — zabrana e-pošte
  false,       // banEmailDomain — zabrana domena e-pošte
  true,        // banIP — zabrana IP adrese
  true,        // deleteAllUsersComments — obriši sve komentare korisnika
  bannedUntil,
  false,       // isShadowBan — skrivena zabrana
  updateId,
  banReason,
  sso
);
[inline-code-end]