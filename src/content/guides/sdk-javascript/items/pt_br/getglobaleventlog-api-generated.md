req
tenantId
urlId
userIdWS

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userIdWS | string | Sim |  |
| startTime | number | Sim |  |
| endTime | number | Não |  |

## Resposta

Retorna: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEventLogs() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const userIdWS: string = "user_ws_abcde";
  const startTime: number = Date.now() - 24 * 60 * 60 * 1000; // 24 horas atrás
  const endTime: number = Date.now();

  const logWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const logWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
}
[inline-code-end]

---