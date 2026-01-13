## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | いいえ |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | いいえ |  |

## レスポンス

戻り値: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 例

[inline-code-attrs-start title = 'addHashTagsBulk の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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