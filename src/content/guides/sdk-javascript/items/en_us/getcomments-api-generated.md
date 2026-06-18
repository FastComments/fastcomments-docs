## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | number | No |  |
| limit | number | No |  |
| skip | number | No |  |
| asTree | boolean | No |  |
| skipChildren | number | No |  |
| limitChildren | number | No |  |
| maxTreeDepth | number | No |  |
| urlId | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |
| fromDate | number | No |  |
| toDate | number | No |  |

## Response

Returns: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Example

[inline-code-attrs-start title = 'getComments Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a12b3";
const response: GetComments200Response = await getComments(tenantId, 1, 20, 0, true, 0, 3, 2, "https://mysite.com/posts/678", undefined, undefined, undefined, undefined, "parent_987", undefined, 1716873600000, 1719552000000);
[inline-code-end]
