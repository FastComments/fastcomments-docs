req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| userIdWS | string | Sim |  |
| startTime | number | Sim |  |
| endTime | number | Não |  |

## Resposta

Retorna: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getGlobalEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f7b2a9c';
const urlId: string = 'article-87c1a2b';
const userIdWS: string = 'ws-1a2b3c4d';
const startTime: number = Date.now() - 60 * 60 * 1000; // há 1 hora
const endTime: number = Date.now();

const responseWithEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
const responseWithoutEnd: GetEventLogResponse = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime);
[inline-code-end]

---