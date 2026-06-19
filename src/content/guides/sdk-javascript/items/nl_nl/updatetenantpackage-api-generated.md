## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Ja |  |

## Respons

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateTenantPackage Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7c9a2f";
const id: string = "pkg_91f2d3b8";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  planId: "business_annual",
  seats: 50,
  autoRenew: true,
  couponCode: "WELCOME2025" // optionele parameter gedemonstreerd
};
const result: APIEmptyResponse = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---