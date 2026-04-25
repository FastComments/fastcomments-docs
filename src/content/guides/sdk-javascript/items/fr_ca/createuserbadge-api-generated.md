## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createUserBadgeParams | CreateUserBadgeParams | Oui |  |

## Réponse

Renvoie : [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const params: CreateUserBadgeParams = {
  name: "Top Contributor",
  slug: "top-contributor",
  description: "Awarded for 100 approved comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top-contributor.png",
  active: true,
  criteria: { approvedComments: 100 },
  customConfig: { showOnProfile: true } // paramètre optionnel
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, params);
[inline-code-end]

---