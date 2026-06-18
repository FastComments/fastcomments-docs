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
const tenantId: string = "tenant_8f3a2b1c4d";
const id: string = "user_62a4f9e0b7";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]
