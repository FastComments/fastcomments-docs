---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createUserBadgeParams | CreateUserBadgeParams | Da |  |

## Odgovor

Vrača: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4b2a";
const createUserBadgeParams: CreateUserBadgeParams = {
  code: "top_contributor",
  title: "Top Contributor",
  description: "Awarded for 100 high-quality comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top_contributor.svg",
  isActive: true,
  criteria: { commentsRequired: 100 },
  customConfig: { displayOnProfile: true } // primer neobveznega parametra
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]

---