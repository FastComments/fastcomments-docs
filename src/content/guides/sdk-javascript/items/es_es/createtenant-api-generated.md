## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createTenantBody | CreateTenantBody | Sí |  |

## Respuesta

Devuelve: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const createTenantBody: CreateTenantBody = {
  name: "Acme Corporation",
  domain: "comments.acme.com",
  adminContact: { name: "Jane Doe", email: "jane.doe@acme.com" },
  billingInfo: { planId: "pro-monthly", billingContactEmail: "billing@acme.com" },
  importedSite: { siteId: "site-123", siteName: "Acme Blog" } // sitio importado opcional
};
const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---