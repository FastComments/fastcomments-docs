## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwoord

Retourneert: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCachedNotificationCount Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const id: string = 'user_00012345';
const includeUnreadOnly: boolean | undefined = true; // optionele parametervlag (gedemonstreerd)
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---