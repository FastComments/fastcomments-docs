## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createTenantPackageBody | CreateTenantPackageBody | はい |  |

## レスポンス

戻り値: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## 例

[inline-code-attrs-start title = 'createTenantPackage の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = 'tenant_acme_01';
  const createTenantPackageBody: CreateTenantPackageBody = {
    packageName: 'Pro Annual',
    seats: 100,
    billingCycle: 'annual',
    autoRenew: true,
    metadata: { region: 'us-west-2' } // 任意のメタデータフィールド
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
run();
[inline-code-end]

---