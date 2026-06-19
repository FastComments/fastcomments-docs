req
tenantId
urlId
userIdWS

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| userIdWS | string | Evet |  |
| startTime | number | Evet |  |
| endTime | number | Hayır |  |

## Yanıt

Döndürür: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getEventLog Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "f2b3d9e8-1c4b-4a7e-9f6d-2b8c3e1a4f5d";
const urlId: string = "news/article/2026/06/18/fastcomments";
const userIdWS: string = "ws-user-78b3ef";
const startTime: number = Date.now() - 24 * 60 * 60 * 1000;
const endTime: number = Date.now();

const responseWithoutEnd: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime);
const responseWithEnd: GetEventLogResponse = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---