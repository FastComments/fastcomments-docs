## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createUserBadgeParams | CreateUserBadgeParams | Oui |  |

## Réponse

Renvoie: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4b2a";
const createUserBadgeParams: CreateUserBadgeParams = {
  code: "top_contributor",
  title: "Top Contributor",
  description: "Awarded for 100 high-quality comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top_contributor.svg",
  isActive: true,
  criteria: { commentsRequired: 100 },
  customConfig: { displayOnProfile: true } // paramètre optionnel démontré
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
[inline-code-end]

---