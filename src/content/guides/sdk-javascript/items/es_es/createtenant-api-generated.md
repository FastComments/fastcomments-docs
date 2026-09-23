## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | Yes |  |

## Respuesta

Devuelve: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-2024";
  const createTenantBody: CreateTenantBody = {
    name: "Acme Corp",
    domainConfiguration: { domain: "comments.acme.com", sslEnabled: true },
    importedSite: { siteId: "site-123", source: "wordpress" },
    billingInfo: { plan: "enterprise", renewalDate: "2025-01-01" },
    description: "Tenant for Acme Corp's production environment", // opcional
  };
  const response: CreateTenantResponse = await createTenant(tenantId, createTenantBody);
}
run();
[inline-code-end]

---