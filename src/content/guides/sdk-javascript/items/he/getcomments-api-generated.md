## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| page | number | לא |  |
| limit | number | לא |  |
| skip | number | לא |  |
| asTree | boolean | לא |  |
| skipChildren | number | לא |  |
| limitChildren | number | לא |  |
| maxTreeDepth | number | לא |  |
| urlId | string | לא |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |
| contextUserId | string | לא |  |
| hashTag | string | לא |  |
| parentId | string | לא |  |
| direction | SortDirections | לא |  |
| fromDate | number | לא |  |
| toDate | number | לא |  |

## תגובה

מוחזר: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]

---