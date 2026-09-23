req
tenantId
urlId
userIdWS

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 是 |  |
| startTime | number | 是 |  |
| endTime | number | 否 |  |

## 响应

返回：[`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## 示例

[inline-code-attrs-start title = 'getEventLog 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "url_9876";
const userIdWS: string = "user_abcde";
const startTime: number = Date.now() - 86_400_000; // 24 小时前
const endTime: number = Date.now();

const fullLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const partialLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]

---