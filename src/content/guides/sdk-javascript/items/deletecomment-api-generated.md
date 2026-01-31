## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| contextUserId | string | No |  |
| isLive | boolean | No |  |

## Response

Returns: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2a1c';
const id: string = 'comment_7a9d2f4b';
const contextUserId: string = 'user_42f1c9';
const isLive: boolean = true;

(async () => {
  const result: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
  console.log(result);
})();
[inline-code-end]
