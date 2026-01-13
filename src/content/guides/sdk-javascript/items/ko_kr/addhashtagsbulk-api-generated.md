---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 아니오 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 아니오 |  |

## 응답

반환: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 예제

[inline-code-attrs-start title = 'addHashTagsBulk 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_3f2b9a';
  const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
    tags: [
      { name: 'performance', description: 'Comments about site performance', visibleToModeratorsOnly: false },
      { name: 'feature-request', description: 'Requests for new features', visibleToModeratorsOnly: true }
    ]
  };
  const result: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
  const resultWithNoTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBody);
  console.log(result, resultWithNoTenant);
})();
[inline-code-end]

---