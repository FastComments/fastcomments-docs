## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-613";
const id: string = "badge-7f9d1a2b-0001";
const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
[inline-code-end]
