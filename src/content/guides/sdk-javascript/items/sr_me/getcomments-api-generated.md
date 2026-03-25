## Параметри

| Име | Тип | Обавезно | Опис |
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

## Одговор

Враћа: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Пример

[inline-code-attrs-start title = 'getComments Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // страница
  20, // ограничење
  0, // прескочи
  true, // као дрво
  1, // прескочи децу
  3, // ограничење за децу
  4, // максимална дубина дрвета
  'articles/2026/new-product-launch', // ид URL-а
  'user_7890', // ид корисника
  'anon_4f3b2', // ид анонимног корисника
  undefined, // ид корисника у контексту
  '#launch', // хаштага
  undefined // ид родитеља
);
[inline-code-end]