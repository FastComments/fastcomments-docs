## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Так |  |
| banEmail | boolean | Ні |  |
| banEmailDomain | boolean | Ні |  |
| banIP | boolean | Ні |  |
| deleteAllUsersComments | boolean | Ні |  |
| bannedUntil | string | Ні |  |
| isShadowBan | boolean | Ні |  |
| updateId | string | Ні |  |
| banReason | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // заблокувати email
  false,       // заблокувати домен email
  true,        // заблокувати IP
  true,        // видалити всі коментарі користувача
  bannedUntil,
  false,       // тіньовий бан
  updateId,
  banReason,
  sso
);
[inline-code-end]