## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 아니오 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 아니오 |  |

## 응답

반환: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 예시

[inline-code-attrs-start title = 'addHashTagsBulk 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 테넌트 식별자 생성 (선택 매개변수)
const tenantId: string = "tenant_9f8c2b7a";

// 개별 태그 항목 준비
const tag1: BulkCreateHashTagsBodyTagsInner = {
  name: "product-feedback",
  label: "Product Feedback",
  color: "#1f8a70",
  description: "User suggestions and enhancement requests",
  isActive: true
};

const tag2: BulkCreateHashTagsBodyTagsInner = {
  name: "bug-report",
  label: "Bug Report",
  color: "#d64545",
  description: "User-reported defects and issues",
  isActive: true
};

// 일괄 생성 본문 (선택 매개변수)
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [tag1, tag2]
};

// 전역 비동기 함수를 호출하고 타입이 지정된 결과를 할당
const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
[inline-code-end]