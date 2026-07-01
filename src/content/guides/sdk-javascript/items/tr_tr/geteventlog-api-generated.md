req
tenantId
urlId
userIdWS

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| userIdWS | string | Evet |  |
| startTime | number | Evet |  |
| endTime | number | Hayır |  |

## Yanıt

Döndürür: [`GetEventLogResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse1.ts)

## Örnek

[inline-code-attrs-start title = 'getEventLog Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_9876";
    const urlId: string = "page_54321";
    const userIdWS: string = "ws_user_1122";
    const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // bir hafta önce
    const endTime: number = Date.now();

    const log: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    const recentLog: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime);
})();
[inline-code-end]

---