## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Cevap

Dönen Değer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateTenantPackage Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7c9a2f";
const id: string = "pkg_91f2d3b8";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  planId: "business_annual",
  seats: 50,
  autoRenew: true,
  couponCode: "WELCOME2025" // isteğe bağlı parametre gösterimi
};
const result: APIEmptyResponse = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---