req
tenantId
urlId
userIdWS

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | number | Ja |  |
| endTime | number | Nein |  |

## Antwort

Rückgabe: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getEventLog Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "url_9876";
const userIdWS: string = "user_abcde";
const startTime: number = Date.now() - 86_400_000; // vor 24 Stunden
const endTime: number = Date.now();

const fullLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const partialLog: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]

---