## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| spam | boolean | Нет |  |
| permNotSpam | boolean | Нет |  |
| broadcastId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример postSetCommentSpamStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42";
const commentId: string = "comment_1001";

const spam: boolean = true;
const permNotSpam: boolean = false;
const broadcastId: string = "broadcast_2023";
const sso: string = "sso_user_5678";

const resultFull: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId,
  spam,
  permNotSpam,
  broadcastId,
  sso
);

const resultMinimal: APIEmptyResponse = await postSetCommentSpamStatus(
  tenantId,
  commentId
);
[inline-code-end]