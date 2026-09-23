solicitud
tenantId
urlId
userIdWS

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| userIdWS | string | Sí |  |
| startTime | number | Sí |  |
| endTime | number | No |  |

## Respuesta

Devuelve: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchEventLogs() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const userIdWS: string = "user_ws_abcde";
  const startTime: number = Date.now() - 24 * 60 * 60 * 1000; // hace 24 horas
  const endTime: number = Date.now();

  const logWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
  const logWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
}
[inline-code-end]

---