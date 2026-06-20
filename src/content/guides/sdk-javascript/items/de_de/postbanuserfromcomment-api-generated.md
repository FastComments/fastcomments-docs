## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| banEmail | boolean | Nein |  |
| banEmailDomain | boolean | Nein |  |
| banIP | boolean | Nein |  |
| deleteAllUsersComments | boolean | Nein |  |
| bannedUntil | string | Nein |  |
| isShadowBan | boolean | Nein |  |
| updateId | string | Nein |  |
| banReason | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Beispiel

[inline-code-attrs-start title = 'postBanUserFromComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // banEmail (E-Mail sperren)
  false,       // banEmailDomain (E-Mail-Domain sperren)
  true,        // banIP (IP-Adresse sperren)
  true,        // deleteAllUsersComments (Alle Kommentare des Nutzers löschen)
  bannedUntil,
  false,       // isShadowBan (Schattenbann)
  updateId,
  banReason,
  sso
);
[inline-code-end]

---