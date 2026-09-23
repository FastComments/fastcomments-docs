## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantBody | UpdateTenantBody | Ja |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'updateTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdateTenant() {
  const tenantId: string = "tenant-abc123";
  const id: string = "config-456def";

  const updateTenantBody: UpdateTenantBody = {
    // erforderliches Feld
    name: "Acme International",
    // optionale Felder können weggelassen werden, z. B. billingInfo, domainConfiguration
  };

  const response: APIEmptyResponse = await updateTenant(tenantId, id, updateTenantBody);
  console.log(response);
}
[inline-code-end]