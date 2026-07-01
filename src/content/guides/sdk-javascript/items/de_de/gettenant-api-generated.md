## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Rückgabe: [`GetTenantResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const id: string = "user_9876";
  const tenantInfo: GetTenantResponse1 = await getTenant(tenantId, id);
  console.log(tenantInfo);
}

fetchTenant();
[inline-code-end]