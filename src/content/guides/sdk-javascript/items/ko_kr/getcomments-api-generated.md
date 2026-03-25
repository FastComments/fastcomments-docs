## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | number | 아니요 |  |
| limit | number | 아니요 |  |
| skip | number | 아니요 |  |
| asTree | boolean | 아니요 |  |
| skipChildren | number | 아니요 |  |
| limitChildren | number | 아니요 |  |
| maxTreeDepth | number | 아니요 |  |
| urlId | string | 아니요 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |
| contextUserId | string | 아니요 |  |
| hashTag | string | 아니요 |  |
| parentId | string | 아니요 |  |
| direction | SortDirections | 아니요 |  |

## 응답

반환: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // 페이지
  20, // 페이지당 항목 수
  0, // 건너뛸 항목 수
  true, // 트리 구조로 반환
  1, // 건너뛸 자식 개수
  3, // 자식 제한
  4, // 트리의 최대 깊이
  'articles/2026/new-product-launch', // urlId
  'user_7890', // 사용자 ID
  'anon_4f3b2', // 익명 사용자 ID
  undefined, // contextUserId
  '#launch', // 해시태그
  undefined // 부모 ID
);
[inline-code-end]

---