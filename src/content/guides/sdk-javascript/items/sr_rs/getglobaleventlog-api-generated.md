req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| userIdWS | string | Да |  |
| startTime | number | Да |  |
| endTime | number | Не |  |

## Одговор

Враћа: [`GetGlobalEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGlobalEventLogResponse.ts)

## Пример

[inline-code-attrs-start title = 'Primer getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const urlId: string = 'article-2023-09-15';
  const userIdWS: string = 'ws_987654321';
  const startTime: number = Date.now() - 86400000;
  const endTime: number = Date.now();

  const fullLog: GetGlobalEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const recentLog: GetGlobalEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);

  console.log(fullLog, recentLog);
})();
[inline-code-end]