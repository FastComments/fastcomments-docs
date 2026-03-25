## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Sí |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3b7f9d-prod";
const id: string = "pkg_enterprise_2026";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: "Enterprise Plus",
  isActive: true,
  // campos opcionales omitidos intencionalmente (p. ej., description, limits)
} as UpdateTenantPackageBody;
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---