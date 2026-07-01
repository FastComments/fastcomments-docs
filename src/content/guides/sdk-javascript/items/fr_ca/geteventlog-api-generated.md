req
tenantId
urlId
userIdWS

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | number | Yes |  |
| endTime | number | No |  |

## Réponse

Renvoie : [`GetEventLogResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_9876";
    const urlId: string = "page_54321";
    const userIdWS: string = "ws_user_1122";
    const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // il y a une semaine
    const endTime: number = Date.now();

    const log: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    const recentLog: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime);
})();
[inline-code-end]

---