## Параметры

| Имя | Тип | Обязательно | Описание |
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
  1, // страница
  20, // лимит
  0, // пропустить
  true, // в виде дерева
  1, // пропустить дочерние
  3, // лимит дочерних
  4, // максимальная глубина дерева
  'articles/2026/new-product-launch', // urlId
  'user_7890', // идентификатор пользователя
  'anon_4f3b2', // идентификатор анонимного пользователя
  undefined, // идентификатор контекстного пользователя
  '#launch', // хэштег
  undefined // идентификатор родителя
);
[inline-code-end]

---