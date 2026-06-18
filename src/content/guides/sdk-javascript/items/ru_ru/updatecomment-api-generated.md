## Параметры

| Имя | Тип | Обязательный | Описание |
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

[inline-code-attrs-start title = 'Пример updateComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3c1b2a';
const commentId: string = 'cmt_8d9f2a4b';
const updatableCommentParams: UpdatableCommentParams = {
  body: 'Updating this comment to clarify the feature behavior and include a timestamp.',
  metadata: { category: 'support', editedReason: 'clarify instructions' },
  visible: true
};
const contextUserId: string = 'user_42';
const doSpamCheck: boolean = true;
const result: FlagCommentPublic200Response = await updateComment(tenantId, commentId, updatableCommentParams, contextUserId, doSpamCheck);
[inline-code-end]

---