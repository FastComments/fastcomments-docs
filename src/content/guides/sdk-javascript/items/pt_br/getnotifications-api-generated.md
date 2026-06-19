## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| userId | string | Não |  |
| urlId | string | Não |  |
| fromCommentId | string | Não |  |
| viewed | boolean | Não |  |
| type | string | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_98b3f';
const userId: string = 'user_8a3f';
const urlId: string = '/blog/2026/new-feature';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 10;
const notifications: GetNotificationsResponse = await getNotifications(tenantId, userId, urlId, undefined, viewed, type, skip);
[inline-code-end]

---