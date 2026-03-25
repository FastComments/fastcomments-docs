---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const id: string = 'user_00012345';
const includeUnreadOnly: boolean | undefined = true; // opciona zastavica parametra (demonstrirano)
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, id);
[inline-code-end]

---