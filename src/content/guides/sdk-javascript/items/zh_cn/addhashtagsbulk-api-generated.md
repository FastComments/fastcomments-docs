---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 否 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 否 |  |

## 响应

返回: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkCreateHashTagsResponse.ts)

## 示例

[inline-code-attrs-start title = 'addHashTagsBulk 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const customConfig: CustomConfigParameters = { displayColor: '#3178C6', priority: 1 };
const tags: BulkCreateHashTagsBodyTagsInner[] = [
  {
    name: 'typescript',
    slug: 'typescript',
    description: 'Questions and examples for TypeScript usage',
    isActive: true,
    customConfig
  }
];
const body: BulkCreateHashTagsBody = { tags };

const responseWithTenant: BulkCreateHashTagsResponse = await addHashTagsBulk(tenantId, body);
const responseWithoutTenant: BulkCreateHashTagsResponse = await addHashTagsBulk(undefined, body);
[inline-code-end]

---