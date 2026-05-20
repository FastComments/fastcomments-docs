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
const tenantId: string = 'acme-inc-001';
const maybeBadgeId: string | undefined = Math.random() > 0.5 ? 'badge-2048' : undefined;
const id: string = maybeBadgeId ?? 'badge-0001';
const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
[inline-code-end]
