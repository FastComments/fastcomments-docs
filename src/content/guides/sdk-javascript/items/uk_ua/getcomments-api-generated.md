## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| page | number | Ні |  |
| limit | number | Ні |  |
| skip | number | Ні |  |
| asTree | boolean | Ні |  |
| skipChildren | number | Ні |  |
| limitChildren | number | Ні |  |
| maxTreeDepth | number | Ні |  |
| urlId | string | Ні |  |
| userId | string | Ні |  |
| anonUserId | string | Ні |  |
| contextUserId | string | Ні |  |
| hashTag | string | Ні |  |
| parentId | string | Ні |  |
| direction | SortDirections | Ні |  |
| fromDate | number | Ні |  |
| toDate | number | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---