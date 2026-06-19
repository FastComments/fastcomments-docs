req
tenantId
urlId
userIdWS

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| userIdWS | string | Tak |  |
| startTime | number | Tak |  |
| endTime | number | Nie |  |

## Odpowiedź

Zwraca: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f7b2a9c';
const urlId: string = 'article-87c1a2b';
const userIdWS: string = 'ws-1a2b3c4d';
const startTime: number = Date.now() - 60 * 60 * 1000; // 1 godzinę temu
const endTime: number = Date.now();

const responseWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const responseWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]