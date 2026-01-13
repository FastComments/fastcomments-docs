---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateTenantBody | UpdateTenantBody | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_01H4ZQ7KABCD";
const id: string = "site_9f8e7d6c";
const apiDomainConfiguration: APIDomainConfiguration = {
  primaryDomain: "comments.acme.com",
  allowSubdomains: true
};
const billingInfo: BillingInfo = {
  planName: "Business",
  billingContactEmail: "billing@acme.com",
  seats: 25
};
const updateTenantBody: UpdateTenantBody = {
  displayName: "Acme Corporation Comments",
  apiDomainConfiguration,
  billingInfo, // parámetro opcional demostrado
  enableModeration: true
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---