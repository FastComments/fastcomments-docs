## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 아니요 |  |
| createHashTagBody | CreateHashTagBody | 아니요 |  |

## 응답

반환: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## 예제

[inline-code-attrs-start title = 'addHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const createHashTagBody: CreateHashTagBody = {
  name: 'feature-request',
  label: 'Feature Request',
  color: '#FF5722',
  enabled: true
};
const response: AddHashTag200Response = await addHashTag(tenantId, createHashTagBody);
const responseWithoutTenant: AddHashTag200Response = await addHashTag(undefined, createHashTagBody);
[inline-code-end]