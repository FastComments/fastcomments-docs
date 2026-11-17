## 매개변수

| 이름 | 타입 | 필수 여부 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니요 |  |

## 응답

반환값: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## 예제

[inline-code-attrs-start title = 'getTenantPackages 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5ad8b1e4-8c1b-4f2a-9d3e-7f6a1b2c3d4e';

// 테넌트 패키지의 첫 페이지 가져오기
const response: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const firstPackage: TenantPackage | undefined = response.packages?.[0];
if (firstPackage?.id) {
  console.log(`First package: ${firstPackage.id} - active: ${firstPackage.active}`);
}

// skip을 사용하여 다음 페이지 가져오기
const skip: number = 10;
const nextPage: GetTenantPackagesResponse = await getTenantPackages(tenantId, skip);
const status: APIStatus = nextPage.status;
console.log(`Fetched ${nextPage.packages?.length ?? 0} packages, status: ${status}`);
[inline-code-end]