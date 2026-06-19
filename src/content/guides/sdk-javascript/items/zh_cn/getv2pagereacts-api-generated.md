## 参数

| 名称 | 类型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## 示例

[inline-code-attrs-start title = 'getV2PageReacts 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-82';
const urlId: string = 'https://www.acmecorp.com/blog/product-launch-2026';
const reacts: GetV2PageReacts = await getV2PageReacts(tenantId, urlId);
console.log(reacts);
[inline-code-end]

---