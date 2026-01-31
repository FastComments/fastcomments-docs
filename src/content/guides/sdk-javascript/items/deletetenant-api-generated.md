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
const tenantId: string = 'tenant_82c9f3b2';
const id: string = 'comment_5f1a2b3c';
const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
const sureToken: string = 'confirm_delete_2026-01-09';
const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sureToken);
[inline-code-end]
