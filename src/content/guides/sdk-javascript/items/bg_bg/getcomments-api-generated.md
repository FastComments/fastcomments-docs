## Параметри

| Name | Type | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| page | number | Не |  |
| limit | number | Не |  |
| skip | number | Не |  |
| asTree | boolean | Не |  |
| skipChildren | number | Не |  |
| limitChildren | number | Не |  |
| maxTreeDepth | number | Не |  |
| urlId | string | Не |  |
| userId | string | Не |  |
| anonUserId | string | Не |  |
| contextUserId | string | Не |  |
| hashTag | string | Не |  |
| parentId | string | Не |  |
| direction | SortDirections | Не |  |

## Отговор

Връща: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Пример

[inline-code-attrs-start title = 'getComments Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // страница
  20, // лимит
  0, // брой за пропускане
  true, // като дърво
  1, // брой деца за пропускане
  3, // лимит за деца
  4, // макс. дълбочина на дървото
  'articles/2026/new-product-launch', // идентификатор на url
  'user_7890', // идентификатор на потребител
  'anon_4f3b2', // идентификатор на анонимен потребител
  undefined, // идентификатор на контекстен потребител
  '#launch', // хаштаг
  undefined // идентификатор на родител
);
[inline-code-end]