---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updatableCommentParams | UpdatableCommentParams | Да |  |
| contextUserId | string | Нет |  |
| doSpamCheck | boolean | Нет |  |
| isLive | boolean | Нет |  |

## Ответ

Возвращает: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-prod-01';
const id: string = 'cmt-000127';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Thanks — I updated the steps to include the missing config flag.',
  isHidden: false
};
const contextUserId: string = 'moderator_77';
const doSpamCheck: boolean = true;
const isLive: boolean = true;
const result: APIEmptyResponse = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---