req
tenantId
urlId
userIdWS

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| userIdWS | string | כן |  |
| startTime | number | כן |  |
| endTime | number | כן |  |

## תגובה

מחזיר: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-84b2f1";
const urlId: string = "article-6721";
const userIdWS: string = "ws-conn-9a3c";
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // לפני 7 ימים
const endTimeOptional: number | undefined = undefined; // סוף טווח זמן אופציונלי
const endTime: number = endTimeOptional ?? Date.now();
const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]