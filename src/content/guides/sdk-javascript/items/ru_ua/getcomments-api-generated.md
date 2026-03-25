## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Нет |  |
| limit | number | Нет |  |
| skip | number | Нет |  |
| asTree | boolean | Нет |  |
| skipChildren | number | Нет |  |
| limitChildren | number | Нет |  |
| maxTreeDepth | number | Нет |  |
| urlId | string | Нет |  |
| userId | string | Нет |  |
| anonUserId | string | Нет |  |
| contextUserId | string | Нет |  |
| hashTag | string | Нет |  |
| parentId | string | Нет |  |
| direction | SortDirections | Нет |  |

## Ответ

Возвращает: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // page — страница
  20, // limit — лимит
  0, // skip — смещение
  true, // asTree — в виде дерева
  1, // skipChildren — пропустить дочерние
  3, // limitChildren — лимит дочерних
  4, // maxTreeDepth — макс. глубина дерева
  'articles/2026/new-product-launch', // urlId — идентификатор URL
  'user_7890', // userId — идентификатор пользователя
  'anon_4f3b2', // anonUserId — анонимный идентификатор пользователя
  undefined, // contextUserId — идентификатор контекстного пользователя
  '#launch', // hashTag — хэштег
  undefined // parentId — идентификатор родителя
);
[inline-code-end]