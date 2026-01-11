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
(async () => {
  const tenantId: string = "acme-corp-tenant-904";
  const id: string = "cmt_7f84d2a9";
  const contextUserId: string = "moderator_21";
  const isLive: boolean = true;
  const result: DeleteComment200Response = await deleteComment(tenantId, id, contextUserId, isLive);
  console.log(result);
})();
[inline-code-end]
