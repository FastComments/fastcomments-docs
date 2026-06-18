req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| userIdWS | string | Да |  |
| startTime | number | Да |  |
| endTime | number | Не |  |

## Отговор

Връща: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Пример

[inline-code-attrs-start title = 'getGlobalEventLog Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_639b7f12";
const urlId: string = "https://www.news-site.com/articles/2026/06/15/important-update-987";
const userIdWS: string = "user_ws_42b7";
const startTime: number = new Date("2026-06-14T00:00:00Z").getTime();
const endTime: number = Date.now();

const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]