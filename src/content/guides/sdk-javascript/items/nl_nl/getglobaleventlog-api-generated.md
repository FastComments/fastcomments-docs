req
tenantId
urlId
userIdWS

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|---------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| userIdWS | string | Ja |  |
| startTime | number | Ja |  |
| endTime | number | Nee |  |

## Respons

Retourneert: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getGlobalEventLog Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEventLogs() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const userIdWS: string = "user_ws_abcde";
  const startTime: number = Date.now() - 24 * 60 * 60 * 1000; // 24 uur geleden
  const endTime: number = Date.now();

  const logWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const logWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
}
[inline-code-end]

---