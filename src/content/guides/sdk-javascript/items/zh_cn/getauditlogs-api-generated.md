## 参数

| 名称 | 类型 | 是否必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| order | SORTDIR | 否 |  |
| after | number | 否 |  |
| before | number | 否 |  |

## 响应

返回: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9a8b7c';
const limit: number = 100;
const skip: number = 0;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 天前
const before: number = Date.now();
const auditLogs: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, undefined, after, before);
[inline-code-end]

---