## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sure | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteTenant Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-84f2';
const id: string = 'delete-9b3f7a';
const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, 'confirm-deletion');
[inline-code-end]
