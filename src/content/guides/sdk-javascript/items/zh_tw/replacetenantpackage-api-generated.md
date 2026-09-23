## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'replaceTenantPackage 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function replacePackageDemo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const packageId: string = "basic-plan-2023";
  const replaceBody: ReplaceTenantPackageBody = {
    newPackageId: "enterprise-plan-2024"
    // 如有需要，可在此加入可選欄位
  };
  const response: APIEmptyResponse = await replaceTenantPackage(tenantId, packageId, replaceBody);
  console.log(response);
}
[inline-code-end]