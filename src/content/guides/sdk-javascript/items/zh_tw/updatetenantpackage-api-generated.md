## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantPackageBody | UpdateTenantPackageBody | 是 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateTenantPackage 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7c9a2f";
const id: string = "pkg_91f2d3b8";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  planId: "business_annual",
  seats: 50,
  autoRenew: true,
  couponCode: "WELCOME2025" // 示範可選參數
};
const result: APIEmptyResponse = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]

---