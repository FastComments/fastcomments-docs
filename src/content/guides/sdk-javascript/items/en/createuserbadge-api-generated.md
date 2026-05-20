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
const tenantId: string = "acme-corp-001";
const createUserBadgeParams: CreateUserBadgeParams = {
  badgeId: "bronze-contributor",
  title: "Bronze Contributor",
  description: "Awarded for first 10 contributions",
  imageUrl: "https://cdn.acme.com/badges/bronze.png",
  isActive: true, // optional parameter example
  metadata: { awardedBy: "community-team" } // optional parameter example
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]
