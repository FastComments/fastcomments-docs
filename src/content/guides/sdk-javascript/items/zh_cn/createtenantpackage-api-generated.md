## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantPackageBody | CreateTenantPackageBody | 是 |  |

## 响应

返回：[`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## 示例

[inline-code-attrs-start title = 'createTenantPackage 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function createTenantPackageExample(): Promise<void> {
  const tenantId: string = '5f2a9b3c8e4d1a2b3c4d5e6f';

  const createTenantPackageBody: CreateTenantPackageBody = {
    name: 'Premium Support Package',
    packageId: 'tenant-premium-annual',
    billingInterval: 'yearly',
    price: 499.99,
    currency: 'USD',
    maxSeats: 25,
    status: 'active'
  };

  const result: CreateTenantPackageResponse = await createTenantPackage(
    tenantId,
    createTenantPackageBody
  );

  const status: APIStatus = result.status;
  const createdPackage: TenantPackage | undefined = result.tenantPackage;

  if (createdPackage) {
    console.log(`Created package ${createdPackage.name} with status ${status}`);
  }
}
[inline-code-end]