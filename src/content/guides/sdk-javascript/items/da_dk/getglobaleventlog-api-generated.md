req
tenantId
urlId
userIdWS

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | number | Ja |  |
| endTime | number | Nej |  |

## Svar

Returnerer: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getGlobalEventLog Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f7b2a9c';
const urlId: string = 'article-87c1a2b';
const userIdWS: string = 'ws-1a2b3c4d';
const startTime: number = Date.now() - 60 * 60 * 1000; // 1 time siden
const endTime: number = Date.now();

const responseWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const responseWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]