## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | boolean | No |  |
| type | string | No |  |
| skip | number | No |  |

## Svar

Returnerer: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getNotifications Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "tenant_42";
  const userId: string = "user_1001";
  const urlId: string = "url_2023";
  const viewed: boolean = true;
  const skip: number = 0;

  const notifications: GetNotificationsResponse = await getNotifications(
    tenantId,
    userId,
    urlId,
    undefined,
    viewed,
    undefined,
    skip
  );

  console.log(notifications);
}
[inline-code-end]

---