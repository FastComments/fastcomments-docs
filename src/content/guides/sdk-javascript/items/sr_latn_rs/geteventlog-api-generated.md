req  
tenantId  
urlId  
userIdWS  

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| userIdWS | string | Da |  |
| startTime | number | Da |  |
| endTime | number | Ne |  |

## Odgovor

Vraća: [`GetEventLogResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse1.ts)

## Primer

[inline-code-attrs-start title = 'Primer getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_9876";
    const urlId: string = "page_54321";
    const userIdWS: string = "ws_user_1122";
    const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // one week ago
    const endTime: number = Date.now();

    const log: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    const recentLog: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime);
})();
[inline-code-end]