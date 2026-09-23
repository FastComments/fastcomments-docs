## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| badgeId | string | Non |  |
| type | number | Non |  |
| displayedOnComments | boolean | Non |  |
| limit | number | Non |  |
| skip | number | Non |  |

## Réponse

Retourne : [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserBadges'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";

  // Tous les paramètres fournis
  const fullResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    "user-42",
    "badge-premium",
    1,
    true,
    10,
    0
  );

  // Seulement les paramètres obligatoires et un paramètre optionnel (limit)
  const limitedResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    undefined,
    undefined,
    undefined,
    undefined,
    5
  );

  console.log(fullResponse, limitedResponse);
})();
[inline-code-end]

---