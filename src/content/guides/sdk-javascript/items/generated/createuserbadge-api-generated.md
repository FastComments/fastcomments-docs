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
const tenantId: string = "fastcomments_tenant_01";
const createUserBadgeParams: CreateUserBadgeParams = {
  name: "Community Helper",
  description: "Awarded for consistently helpful and constructive replies",
  iconUrl: "https://assets.examplecdn.com/badges/community-helper.svg",
  // optional parameters
  criteria: { type: "comment_count", threshold: 100 },
  active: true,
  customConfig: { color: "#1e90ff", highlight: true }
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]
