## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | はい |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'replaceTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function replacePackageDemo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const packageId: string = "basic-plan-2023";
  const replaceBody: ReplaceTenantPackageBody = {
    newPackageId: "enterprise-plan-2024"
    // 必要に応じてここにオプションフィールドを追加できます
  };
  const response: APIEmptyResponse = await replaceTenantPackage(tenantId, packageId, replaceBody);
  console.log(response);
}
[inline-code-end]