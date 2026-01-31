## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-corp-01';
  const id: string = 'gold-badge-0042';
  interface BadgeQueryOptions { includeInactive?: boolean; }
  const options: BadgeQueryOptions = { includeInactive: false };
  const result: GetUserBadge200Response = await getUserBadge(tenantId, id);
  console.log(result, options);
})();
[inline-code-end]
