## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 否 |  |
| createHashTagBody | CreateHashTagBody | 否 |  |

## 回應

回傳：[`AddHashTag200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTag200Response.ts)

## 範例

[inline-code-attrs-start title = 'addHashTag 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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