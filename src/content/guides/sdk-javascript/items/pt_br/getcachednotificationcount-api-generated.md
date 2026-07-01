## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetCachedNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCountResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'getCachedNotificationCount Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "e3f1a9b2-4c5d-6e7f-8a9b-0c1d2e3f4a5b";
const userId: string = "u-20231101-001";

const notificationResult: GetCachedNotificationCountResponse1 = await getCachedNotificationCount(tenantId, userId);

console.log(notificationResult);
[inline-code-end]