## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTenant 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp";
const id: string = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
interface GetOptions { includeDeleted?: boolean; locale?: string; }
const options: GetOptions = { locale: "en-US" };
const result: GetTenant200Response = await getTenant(tenantId, id);
[inline-code-end]

---