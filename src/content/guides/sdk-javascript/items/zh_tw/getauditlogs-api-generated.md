## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| order | SORTDIR | 否 |  |
| after | number | 否 |  |
| before | number | 否 |  |

## 回應

回傳: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getAuditLogs 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f9a4';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = SORTDIR.DESC;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 天前
const auditLogsResponse: GetAuditLogsResponse = await getAuditLogs(tenantId, limit, skip, order, after);
console.log((auditLogsResponse as unknown) ? 'Audit logs fetched' : 'No logs');
[inline-code-end]

---