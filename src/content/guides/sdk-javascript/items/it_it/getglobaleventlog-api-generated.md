req
tenantId
urlId
userIdWS

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| userIdWS | string | Sì |  |
| startTime | number | Sì |  |
| endTime | number | No |  |

## Risposta

Restituisce: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f7b2a9c';
const urlId: string = 'article-87c1a2b';
const userIdWS: string = 'ws-1a2b3c4d';
const startTime: number = Date.now() - 60 * 60 * 1000; // 1 ora fa
const endTime: number = Date.now();

const responseWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const responseWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]