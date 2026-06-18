## Параметри

| Назив | Тип | Обавезно | Опис |
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
| fromDate | number | Не |  |
| toDate | number | Не |  |

## Одговор

Враћа: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Пример

[inline-code-attrs-start title = 'getComments Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---