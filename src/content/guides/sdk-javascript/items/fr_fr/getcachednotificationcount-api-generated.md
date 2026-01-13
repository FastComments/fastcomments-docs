## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple pour getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const baseNotificationId: string = 'notif-000123';
const idSuffix: string | undefined = undefined; // optional parameter example
const notificationId: string = idSuffix ? `${baseNotificationId}-${idSuffix}` : baseNotificationId;
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, notificationId);
console.log(result);
[inline-code-end]

---