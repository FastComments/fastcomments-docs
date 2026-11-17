## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 否 |  |

## 回應

回傳:[`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkCreateHashTagsResponse.ts)

## 範例

[inline-code-attrs-start title = 'addHashTagsBulk 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '54f2c8d9a1b2c3d4e5f60718';

const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { tag: 'customer-support' },
    { tag: 'product-feedback' }
  ]
};

const response: BulkCreateHashTagsResponse = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);

const status: APIStatus = response.status;
const createdTags: TenantHashTag[] = response.results.map(
  (result: BulkCreateHashTagsResponseResultsInner) => result.hashTag as TenantHashTag
);
console.log(status, createdTags.length);
[inline-code-end]