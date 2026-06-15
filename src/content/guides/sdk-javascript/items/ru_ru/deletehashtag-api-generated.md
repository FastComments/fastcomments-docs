---
## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tag | string | Да |  |
| tenantId | string | Нет |  |
| deleteHashTagRequest | DeleteHashTagRequest | Нет |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "breaking-news";
const tenantId: string = "tenant_72a1";
const deleteHashTagRequest: DeleteHashTagRequest = {
  reason: "consolidate-duplicates",
  requestedBy: "moderator@dailypress.com",
  forceDelete: true
};
const result: FlagCommentPublic200Response = await deleteHashTag(tag, tenantId, deleteHashTagRequest);
[inline-code-end]

---