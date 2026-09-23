## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantBody | UpdateTenantBody | Yes |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdateTenant() {
  const tenantId: string = "tenant-abc123";
  const id: string = "config-456def";

  const updateTenantBody: UpdateTenantBody = {
    // campo requerido
    name: "Acme International",
    // los campos opcionales pueden omitirse, p.ej., billingInfo, domainConfiguration
  };

  const response: APIEmptyResponse = await updateTenant(tenantId, id, updateTenantBody);
  console.log(response);
}
[inline-code-end]

---