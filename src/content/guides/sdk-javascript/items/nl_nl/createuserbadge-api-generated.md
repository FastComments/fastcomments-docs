---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenantId | string | Ja |  |
| createUserBadgeParams | CreateUserBadgeParams | Ja |  |

## Respons

Retourneert: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createUserBadge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // optionele criteria
  displayOrder: 10,
  metadata: { featured: true } // optionele metadata
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---