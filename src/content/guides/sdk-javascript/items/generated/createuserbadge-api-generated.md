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
const tenantId: string = 'tenant_9f8b6c';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  description: 'Awarded to users with 500+ helpful replies',
  imageUrl: 'https://cdn.fastcomments.com/badges/top-contributor.png',
  isActive: true,
  criteria: { minComments: 500, minUpvotes: 120 }, // optional criteria
  customConfig: { badgeColor: '#FFD700', displayOnProfile: true } // optional custom config
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]
