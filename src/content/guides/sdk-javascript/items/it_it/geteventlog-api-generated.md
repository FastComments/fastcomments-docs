req
tenantId
urlId
userIdWS

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| userIdWS | string | Sì |  |
| startTime | number | Sì |  |
| endTime | number | No |  |

## Risposta

Restituisce: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3a2b';
const urlId: string = 'news/2026/06/fastcomments-release';
const userIdWS: string = 'ws_user_48291';
const startTime: number = Date.now() - 86_400_000;
const endTime: number = Date.now();
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---