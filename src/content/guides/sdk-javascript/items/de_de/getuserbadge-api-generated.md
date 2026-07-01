## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Rückgabe: [`GetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserBadge Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
  const tenantId: string = "acme-corp-tenant-001";
  const badgeId: string = "badge-5f9d3a2b";

  const badgeResponse: GetUserBadgeResponse = await getUserBadge(tenantId, badgeId);

  // Optionale Felder sicher zugreifen
  const badgeName: string | undefined = badgeResponse.userBadge?.name;
  console.log(`Badge ID: ${badgeId}, Name: ${badgeName ?? "Unnamed"}`);
}

runExample();
[inline-code-end]