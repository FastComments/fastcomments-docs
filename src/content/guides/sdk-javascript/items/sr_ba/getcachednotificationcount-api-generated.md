---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Primjer

[inline-code-attrs-start title = 'getCachedNotificationCount Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const id: string = 'user_00012345';
const includeUnreadOnly: boolean | undefined = true; // opciona zastavica parametra (prikazano)
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---