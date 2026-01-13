## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 아니요 |  |
| createHashTagBody | CreateHashTagBody | 아니요 |  |

## 응답

반환: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## 예제

[inline-code-attrs-start title = 'addHashTag 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b2f6c2b';
const createBody: CreateHashTagBody = {
  tag: 'feature-request',
  label: 'Feature Request',
  description: 'Requests for new functionality in the web client',
  isActive: true,
  visibility: 'public',
  allowedDomains: ['example.com', 'internal.example.com']
};
const result: AddHashTag200Response = await addHashTag(tenantId, createBody);
const resultWithoutTenant: AddHashTag200Response = await addHashTag(undefined, {
  tag: 'bug',
  label: 'Bug',
  description: 'Use for reproducible bugs reported by users',
  isActive: true,
  visibility: 'public'
});
[inline-code-end]

---