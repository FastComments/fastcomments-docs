## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 応答

返却: [`DeleteTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantPackageResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeTenantPackage(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const packageId: string = "pkg_67890";

  const result: DeleteTenantPackageResponse = await deleteTenantPackage(tenantId, packageId);
  // 必要に応じて結果を使用
}

removeTenantPackage();
[inline-code-end]