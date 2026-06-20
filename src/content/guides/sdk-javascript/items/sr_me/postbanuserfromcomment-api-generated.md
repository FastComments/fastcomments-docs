## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Да |  |
| banEmail | boolean | Не |  |
| banEmailDomain | boolean | Не |  |
| banIP | boolean | Не |  |
| deleteAllUsersComments | boolean | Не |  |
| bannedUntil | string | Не |  |
| isShadowBan | boolean | Не |  |
| updateId | string | Не |  |
| banReason | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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