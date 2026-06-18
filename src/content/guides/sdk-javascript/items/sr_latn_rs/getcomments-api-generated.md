## Parametri

| Name | Type | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| page | number | Ne |  |
| limit | number | Ne |  |
| skip | number | Ne |  |
| asTree | boolean | Ne |  |
| skipChildren | number | Ne |  |
| limitChildren | number | Ne |  |
| maxTreeDepth | number | Ne |  |
| urlId | string | Ne |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |
| contextUserId | string | Ne |  |
| hashTag | string | Ne |  |
| parentId | string | Ne |  |
| direction | SortDirections | Ne |  |
| fromDate | number | Ne |  |
| toDate | number | Ne |  |

## Odgovor

Vraća: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---