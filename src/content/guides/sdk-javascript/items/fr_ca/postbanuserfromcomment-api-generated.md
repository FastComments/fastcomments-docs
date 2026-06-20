## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
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

[inline-code-attrs-start title = 'Exemple de postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // bannir le courriel
  false,       // bannir le domaine du courriel
  true,        // bannir l'IP
  true,        // supprimer tous les commentaires de l'utilisateur
  bannedUntil,
  false,       // bannissement discret
  updateId,
  banReason,
  sso
);
[inline-code-end]