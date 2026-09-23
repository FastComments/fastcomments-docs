## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Retourne : [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCountResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const id: string = "user_9876";
  const response: GetCachedNotificationCountResponse = await getCachedNotificationCount(tenantId, id);
})();
[inline-code-end]

---