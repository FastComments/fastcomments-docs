## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | לא |  |

## תגובה

מחזיר: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkCreateHashTagsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-addHashTagsBulk'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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