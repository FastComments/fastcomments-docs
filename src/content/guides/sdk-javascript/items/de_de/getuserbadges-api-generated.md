## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| badgeId | string | No |  |
| type | number | No |  |
| displayedOnComments | boolean | No |  |
| limit | number | No |  |
| skip | number | No |  |

## Antwort

Rückgabe: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserBadges Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";

  // Alle Parameter angegeben
  const fullResponse: APIGetUserBadgesResponse = await getUserBadges(
    tenantId,
    "user-42",
    "badge-premium",
    1,
    true,
    10,
    0
  );

  // Nur erforderliche und ein optionaler Parameter (limit)
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