## 매개변수

| 이름 | Type | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| meta | string | 아니오 |  |
| skip | number | 아니오 |  |

## 응답

반환: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getTenants 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]

---