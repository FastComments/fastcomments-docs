## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 否 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 否 |  |

## 响应

返回：[`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddHashTagsBulk200Response.ts)

## 示例

[inline-code-attrs-start title = 'addHashTagsBulk 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_01';
const bulkCreateHashTagsBody: BulkCreateHashTagsBody = {
  tags: [
    { name: 'feature-request', slug: 'feature-request', description: 'Requests for new capabilities', isActive: true, customConfig: { visibility: 'public' } as unknown as CustomConfigParameters }
  ]
};
const addHashTagsResponse: AddHashTagsBulk200Response = await addHashTagsBulk(tenantId, bulkCreateHashTagsBody);

const bulkCreateHashTagsBodyNoTenant: BulkCreateHashTagsBody = {
  tags: [
    { name: 'ux-feedback', slug: 'ux-feedback', description: 'User experience suggestions', isActive: true }
  ]
};
const addHashTagsResponseNoTenant: AddHashTagsBulk200Response = await addHashTagsBulk(undefined, bulkCreateHashTagsBodyNoTenant);
[inline-code-end]

---