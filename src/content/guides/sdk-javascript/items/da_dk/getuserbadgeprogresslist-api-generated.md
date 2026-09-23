## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Nej |  |
| limit | number | Nej |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressListResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getUserBadgeProgressList Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const userId: string = "user-123e4567-e89b-12d3-a456-426614174000";
  const limit: number = 10;
  const skip: number = 0;

  const response: APIGetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId, userId, limit, skip);
  const status: APIStatus = response.status;
  const badges: UserBadgeProgress[] = response.badges;
})();
[inline-code-end]