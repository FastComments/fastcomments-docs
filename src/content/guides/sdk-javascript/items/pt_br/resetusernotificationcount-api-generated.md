## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8a3f2b6c";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyX2QxMjM0IiwiaWF0IjoxNjI1MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
const resetResponseWithSso: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId, ssoToken);
const resetResponseWithoutSso: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId);
[inline-code-end]

---