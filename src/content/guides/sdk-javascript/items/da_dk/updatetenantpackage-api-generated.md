## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'Eksempel på updateTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3b7f9d-prod";
const id: string = "pkg_enterprise_2026";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: "Enterprise Plus",
  isActive: true,
  // valgfrie felter udeladt med vilje (f.eks. beskrivelse, grænser)
} as UpdateTenantPackageBody;
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---