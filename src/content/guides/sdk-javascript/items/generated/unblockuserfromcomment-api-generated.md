## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'unBlockUserFromComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9c2f7b';
  const id: string = 'comment_6a1b2c3d';
  const unBlockFromCommentParams: UnBlockFromCommentParams = {
    reason: 'User appealed; evidence reviewed and ban lifted',
    liftedByAdminId: 'admin_42',
    effectiveAt: new Date().toISOString()
  };
  const userId: string = 'user_1024';
  const anonUserId: string = 'anon_7f9e';
  const response: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId, anonUserId);
  console.log(response);
})();
[inline-code-end]
