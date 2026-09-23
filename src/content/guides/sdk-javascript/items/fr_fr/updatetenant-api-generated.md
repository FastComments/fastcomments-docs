---
## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantBody | UpdateTenantBody | Yes |  |

## Réponse

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdateTenant() {
  const tenantId: string = "tenant-abc123";
  const id: string = "config-456def";

  const updateTenantBody: UpdateTenantBody = {
    // champ requis
    name: "Acme International",
    // les champs optionnels peuvent être omis, par ex., billingInfo, domainConfiguration
  };

  const response: APIEmptyResponse = await updateTenant(tenantId, id, updateTenantBody);
  console.log(response);
}
[inline-code-end]

---