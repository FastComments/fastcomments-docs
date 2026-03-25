req
tenantId
urlId
userIdWS

## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 是 |  |
| startTime | number | 是 |  |
| endTime | number | 是 |  |

## 回應

回傳: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## 範例

[inline-code-attrs-start title = 'getGlobalEventLog 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-84b2f1";
const urlId: string = "article-6721";
const userIdWS: string = "ws-conn-9a3c";
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 7 天前
const endTimeOptional: number | undefined = undefined; // 可選的時間範圍結束
const endTime: number = endTimeOptional ?? Date.now();
const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---