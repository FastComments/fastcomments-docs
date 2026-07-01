---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| userId | string | Nein |  |
| limit | number | Nein |  |
| skip | number | Nein |  |

## Antwort

Rückgabe: [`GetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressListResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserBadgeProgressList Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadgeProgress() {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe@example.com";
  const limit: number = 10;
  const skip: number = 5;

  const fullList: GetUserBadgeProgressListResponse = await getUserBadgeProgressList(
    tenantId,
    userId,
    limit,
    skip
  );

  const simpleList: GetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId);
}

fetchBadgeProgress();
[inline-code-end]

---