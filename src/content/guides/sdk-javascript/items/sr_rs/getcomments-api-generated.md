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

[inline-code-attrs-start title = 'Пример getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // страница
  20, // лимит
  0, // прескочи
  true, // у виду стабла
  1, // прескочи подкоментаре
  3, // ограничење подкоментара
  4, // максимална дубина стабла
  'articles/2026/new-product-launch', // ид URL-а
  'user_7890', // ид корисника
  'anon_4f3b2', // анонимни ид корисника
  undefined, // контекстуални ид корисника
  '#launch', // хештег
  undefined // ид родитеља
);
[inline-code-end]

---