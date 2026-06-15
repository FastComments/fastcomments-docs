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

[inline-code-attrs-start title = 'Пример за getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3a2b';
const urlId: string = 'news/2026/06/fastcomments-release';
const userIdWS: string = 'ws_user_48291';
const startTime: number = Date.now() - 86_400_000;
const endTime: number = Date.now();
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---