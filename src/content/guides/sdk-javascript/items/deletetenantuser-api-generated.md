## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_4c9e2b70";
  const id: string = "user_9a1b2c3d";
  const deleteComments: string = "true";
  const commentDeleteMode: string = "cascade";
  const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
  console.log(result);
})();
[inline-code-end]
