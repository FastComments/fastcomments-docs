## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| afterCreatedAt | number | Não |  |
| unreadOnly | boolean | Não |  |
| dmOnly | boolean | Não |  |
| noDm | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_4a9f12";
const afterId: string = "notification_87213";
const afterCreatedAt: number = Math.floor(Date.now() / 1000) - 3600;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.payload";
const result: ResetUserNotifications200Response = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, undefined, sso);
[inline-code-end]