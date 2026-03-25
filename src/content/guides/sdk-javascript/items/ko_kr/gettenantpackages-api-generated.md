## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTenantPackages 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f8e3b4c";
const skip: number = 20;
const packagesDefault: GetTenantPackages200Response = await getTenantPackages(tenantId);
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
[inline-code-end]

---