## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| banEmail | boolean | Hayır |  |
| banEmailDomain | boolean | Hayır |  |
| banIP | boolean | Hayır |  |
| deleteAllUsersComments | boolean | Hayır |  |
| bannedUntil | string | Hayır |  |
| isShadowBan | boolean | Hayır |  |
| updateId | string | Hayır |  |
| banReason | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // banEmail
  false,       // banEmailDomain
  true,        // banIP
  true,        // deleteAllUsersComments
  bannedUntil,
  false,       // isShadowBan
  updateId,
  banReason,
  sso
);
[inline-code-end]

---