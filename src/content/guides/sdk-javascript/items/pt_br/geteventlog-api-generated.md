req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userIdWS | string | Sim |  |
| startTime | number | Sim |  |
| endTime | number | Não |  |

## Resposta

Retorna: [`GetEventLogResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "tenant_9876";
    const urlId: string = "page_54321";
    const userIdWS: string = "ws_user_1122";
    const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // uma semana atrás
    const endTime: number = Date.now();

    const log: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
    const recentLog: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime);
})();
[inline-code-end]