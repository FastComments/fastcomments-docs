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
const tenantId: string = "tenant_987";
const page: number = 1;
const limit: number = 20;
const skip: number = 0;
const asTree: boolean = true;
const skipChildren: number = 0;
const limitChildren: number = 10;
const maxTreeDepth: number = 2;
const urlId: string | undefined = "/news/article-2026-05";
const userId: string | undefined = "user_42";
const hashTag: string | undefined = "product-release";

const result: GetComments200Response = await getComments(
  tenantId,
  page,
  limit,
  skip,
  asTree,
  skipChildren,
  limitChildren,
  maxTreeDepth,
  urlId,
  userId,
  undefined,
  undefined,
  hashTag
);
[inline-code-end]
