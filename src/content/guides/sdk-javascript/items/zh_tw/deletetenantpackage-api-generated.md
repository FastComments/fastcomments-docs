## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

返回：[`DeleteTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantPackageResponse.ts)

## 示例

[inline-code-attrs-start title = 'deleteTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeTenantPackage(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const packageId: string = "pkg_67890";

  const result: DeleteTenantPackageResponse = await deleteTenantPackage(tenantId, packageId);
  // 按需使用結果
}

removeTenantPackage();
[inline-code-end]

---