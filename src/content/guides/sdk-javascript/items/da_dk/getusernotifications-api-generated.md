## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Nej |  |
| pageSize | number | Nej |  |
| afterId | string | Nej |  |
| includeContext | boolean | Nej |  |
| afterCreatedAt | number | Nej |  |
| unreadOnly | boolean | Nej |  |
| dmOnly | boolean | Nej |  |
| noDm | boolean | Nej |  |
| includeTranslations | boolean | Nej |  |
| includeTenantNotifications | boolean | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getUserNotifications Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "acme-corp";
  const urlId: string = "article-42";
  const pageSize: number = 20;
  const includeContext: boolean = true;
  const unreadOnly: boolean = true;

  const notifications: GetMyNotificationsResponse = await getUserNotifications(
    tenantId,
    urlId,
    pageSize,
    undefined,
    includeContext,
    undefined,
    unreadOnly
  );

  console.log(notifications);
}
[inline-code-end]

---