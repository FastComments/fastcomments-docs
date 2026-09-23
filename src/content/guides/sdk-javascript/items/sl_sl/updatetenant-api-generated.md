## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateTenantBody | UpdateTenantBody | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'updateTenant Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdateTenant() {
  const tenantId: string = "tenant-abc123";
  const id: string = "config-456def";

  const updateTenantBody: UpdateTenantBody = {
    // obvezno polje
    name: "Acme International",
    // neobvezna polja je mogoče izpustiti, npr. billingInfo, domainConfiguration
  };

  const response: APIEmptyResponse = await updateTenant(tenantId, id, updateTenantBody);
  console.log(response);
}
[inline-code-end]