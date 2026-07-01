## Paramètres

| Nom | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const id: string = "user_9876";
  const tenantInfo: GetTenantResponse1 = await getTenant(tenantId, id);
  console.log(tenantInfo);
}

fetchTenant();
[inline-code-end]

---