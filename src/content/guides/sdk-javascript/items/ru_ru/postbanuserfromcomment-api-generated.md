## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| banEmail | boolean | Нет |  |
| banEmailDomain | boolean | Нет |  |
| banIP | boolean | Нет |  |
| deleteAllUsersComments | boolean | Нет |  |
| bannedUntil | string | Нет |  |
| isShadowBan | boolean | Нет |  |
| updateId | string | Нет |  |
| banReason | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BanUserFromCommentResult.ts)

## Пример

[inline-code-attrs-start title = 'Пример postBanUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_9f8a7b4e";
const bannedUntil: string = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString();
const sso: string = "sso-user-7f3b2c";
const updateId: string = "upd_20260619_001";
const banReason: string = "Repeated harassment across multiple threads";
const result: BanUserFromCommentResult = await postBanUserFromComment(
  commentId,
  true,        // banEmail — запретить адрес электронной почты
  false,       // banEmailDomain — запретить домен электронной почты
  true,        // banIP — запретить IP-адрес
  true,        // deleteAllUsersComments — удалить все комментарии пользователя
  bannedUntil,
  false,       // isShadowBan — теневой бан
  updateId,
  banReason,
  sso
);
[inline-code-end]