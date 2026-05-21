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
(async () => {
  const tenantId: string = "tenant_sf_001";
  const id: string = "pkg-premium-v2";
  const updateTenantPackageBody: UpdateTenantPackageBody = {
    name: "San Francisco Premium",
    enabled: true,
    customConfig: { maxComments: 500 },
    tosConfig: { required: true } // campos opcionales demostrados por su presencia; los demás omitidos
  } as UpdateTenantPackageBody;
  const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
  console.log(result);
})();
[inline-code-end]

---