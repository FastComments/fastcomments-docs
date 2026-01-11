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
(async () => {
  const tenantId: string = 'acme-inc-7a1b2c';
  const optionalBadgeId: string | undefined = 'badge_9f3e1'; // could be undefined to use fallback
  const id: string = optionalBadgeId ?? 'badge_default_0001';
  const result: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
  console.log(result);
})();
[inline-code-end]
