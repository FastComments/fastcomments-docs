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

## Response

Returns: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Example

[inline-code-attrs-start title = 'getComments Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f2b9a";
const page: number = 1;
const limit: number = 20;
const asTree: boolean = true;
const maxTreeDepth: number = 2;
const urlId: string = "news/2026/05/20/new-feature";
const userId: string = "user_8d34";
const result: GetComments200Response = await getComments(
  tenantId,
  page,
  limit,
  undefined,
  asTree,
  undefined,
  undefined,
  maxTreeDepth,
  urlId,
  userId
);
[inline-code-end]
