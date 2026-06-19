req
tenantId
urlId
userIdWS

## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | number | Yes |  |
| endTime | number | No |  |

## Ответ

Возвращает: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f7b2a9c';
const urlId: string = 'article-87c1a2b';
const userIdWS: string = 'ws-1a2b3c4d';
const startTime: number = Date.now() - 60 * 60 * 1000; // 1 час назад
const endTime: number = Date.now();

const responseWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const responseWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]