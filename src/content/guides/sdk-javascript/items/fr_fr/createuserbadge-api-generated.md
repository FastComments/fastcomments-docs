## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| createUserBadgeParams | CreateUserBadgeParams | Oui |  |

## Réponse

Retourne : [`CreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadgeResponse.ts)

## Exemple

[inline-code-attrs-start title = 'createUserBadge Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "company-42";

const badgeParams: CreateUserBadgeParams = {
  name: "Community Champion",
  iconUrl: "https://assets.example.com/badges/champion.png",
  // la description est facultative et omise ici
};

const result: CreateUserBadgeResponse = await createUserBadge(tenantId, badgeParams);
[inline-code-end]