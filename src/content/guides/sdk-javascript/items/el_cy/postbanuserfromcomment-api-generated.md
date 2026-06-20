## Παράμετροι

| Name | Type | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| commentId | string | Ναι |  |
| banEmail | boolean | Όχι |  |
| banEmailDomain | boolean | Όχι |  |
| banIP | boolean | Όχι |  |
| deleteAllUsersComments | boolean | Όχι |  |
| bannedUntil | string | Όχι |  |
| isShadowBan | boolean | Όχι |  |
| updateId | string | Όχι |  |
| banReason | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // απαγόρευση email
  false,       // απαγόρευση domain email
  true,        // απαγόρευση IP
  true,        // διαγραφή όλων των σχολίων του χρήστη
  bannedUntil,
  false,       // κρυφή απαγόρευση
  updateId,
  banReason,
  sso
);
[inline-code-end]

---