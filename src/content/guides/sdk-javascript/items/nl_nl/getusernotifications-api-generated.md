## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Nee |  |
| pageSize | number | Nee |  |
| afterId | string | Nee |  |
| includeContext | boolean | Nee |  |
| afterCreatedAt | number | Nee |  |
| unreadOnly | boolean | Nee |  |
| dmOnly | boolean | Nee |  |
| noDm | boolean | Nee |  |
| includeTranslations | boolean | Nee |  |
| includeTenantNotifications | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUserNotifications Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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