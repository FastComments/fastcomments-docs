## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| createUserBadgeParams | CreateUserBadgeParams | Ja |  |

## Response

Retourneert: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createUserBadge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const params: CreateUserBadgeParams = {
  name: "Top Contributor",
  slug: "top-contributor",
  description: "Awarded for 100 approved comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top-contributor.png",
  active: true,
  criteria: { approvedComments: 100 },
  customConfig: { showOnProfile: true } // optional parameter
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, params);
[inline-code-end]

---