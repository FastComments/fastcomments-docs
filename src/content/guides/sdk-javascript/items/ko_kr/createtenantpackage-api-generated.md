## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantPackageBody | CreateTenantPackageBody | 예 |  |

## 응답

반환: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## 예제

[inline-code-attrs-start title = 'createTenantPackage 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const tenantId: string = 'tenant_acme_01';
  const createTenantPackageBody: CreateTenantPackageBody = {
    packageName: 'Pro Annual',
    seats: 100,
    billingCycle: 'annual',
    autoRenew: true,
    metadata: { region: 'us-west-2' } // 선택적 메타데이터 필드
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
run();
[inline-code-end]

---