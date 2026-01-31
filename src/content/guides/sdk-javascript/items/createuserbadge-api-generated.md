## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Response

Returns: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'createUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_456';
const metadata: CustomConfigParameters = { themeColor: '#FFD700', release: '2026-01' };
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Commenter',
  slug: 'top-commenter',
  iconUrl: 'https://assets.acme.com/badges/top-commenter.svg',
  // optional fields
  description: 'Awarded to users who post 100+ thoughtful comments',
  isActive: true,
  criteria: { minComments: 100 },
  metadata
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]
