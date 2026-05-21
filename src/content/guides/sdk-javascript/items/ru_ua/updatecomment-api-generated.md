## Параметры

| Name | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updatableCommentParams | UpdatableCommentParams | Да |  |
| contextUserId | string | Нет |  |
| doSpamCheck | boolean | Нет |  |
| isLive | boolean | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3f47b2a1";
const id: string = "comment_9a12b3c4";
const updatableCommentParams: UpdatableCommentParams = {
  body: "Thanks for the update — I've adjusted my view accordingly."
};
const contextUserId: string = "user_8721";
const doSpamCheck: boolean = true;
const isLive: boolean = false;
const result: FlagCommentPublic200Response = await updateComment(tenantId, id, updatableCommentParams, contextUserId, doSpamCheck, isLive);
[inline-code-end]

---