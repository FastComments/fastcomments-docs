## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |
| order | SORTDIR | 아니오 |  |
| after | number | 아니오 |  |
| before | number | 아니오 |  |

## 응답

반환: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## 예제

[inline-code-attrs-start title = 'getAuditLogs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f8d7c3a';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = 'DESC' as SORTDIR;
const after: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 일주일 전
const before: number = Date.now();
const result: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, order, after, before);
[inline-code-end]

---