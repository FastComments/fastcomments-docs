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
(async () => {
  const tenantId: string = 'tenant_12345';
  const result: GetComments200Response = await getComments(
    tenantId,
    1,                     // page
    25,                    // limit
    0,                     // skip
    true,                  // asTree
    0,                     // skipChildren
    3,                     // limitChildren
    5,                     // maxTreeDepth
    'news-article-20251122-abc123', // urlId
    'user_7890',           // userId
    'anon_456',            // anonUserId
    'moderator_101',       // contextUserId
    'discussion',          // hashTag
    'comment_9876',        // parentId
    SortDirections.DESC    // direction
  );
  console.log(result);
})();
[inline-code-end]
