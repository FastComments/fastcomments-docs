запит  
tenantId  
urlId  
userIdWS  

## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| userIdWS | string | Так |  |
| startTime | number | Так |  |
| endTime | number | Ні |  |

## Відповідь

Повертає: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
async function fetchEventLogs() {  
  const tenantId: string = "tenant_12345";  
  const urlId: string = "url_9876";  
  const userIdWS: string = "user_ws_abcde";  
  const startTime: number = Date.now() - 24 * 60 * 60 * 1000; // 24 години тому  
  const endTime: number = Date.now();  

  const logWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);  
  const logWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);  
}  
[inline-code-end]  

---