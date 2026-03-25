## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3b7f9d-prod";
const id: string = "pkg_enterprise_2026";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: "Enterprise Plus",
  isActive: true,
  // neobavezna polja su namjerno izostavljena (npr. description, limits)
} as UpdateTenantPackageBody;
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---