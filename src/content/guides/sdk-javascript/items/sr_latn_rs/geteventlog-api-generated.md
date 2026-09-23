req
tenantId
urlId
userIdWS

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| userIdWS | string | Da |  |
| startTime | number | Da |  |
| endTime | number | Ne |  |

## Odgovor

Vraća: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Primer

[inline-code-attrs-start title = 'getEventLog Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "url_9876";
const userIdWS: string = "user_abcde";
const startTime: number = Date.now() - 86_400_000; // 24 sata ranije
const endTime: number = Date.now();

const fullLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const partialLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]

---