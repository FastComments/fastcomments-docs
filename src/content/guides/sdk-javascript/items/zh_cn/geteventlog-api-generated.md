req
tenantId
urlId
userIdWS

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 是 |  |
| startTime | number | 是 |  |
| endTime | number | 是 |  |

## 响应

返回：[`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## 示例

[inline-code-attrs-start title = 'getEventLog 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const urlId: string = 'article-2026-03-25';
const userIdWS: string | undefined = undefined; // 可选的上游值
const startTime: number = Date.parse('2026-03-01T00:00:00Z');
const endTime: number = Date.parse('2026-03-25T23:59:59Z');

const eventLogResponse: GetEventLog200Response = await getEventLog(
  tenantId,
  urlId,
  userIdWS ?? 'ws_user_8b1f',
  startTime,
  endTime
);
[inline-code-end]