---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example(): Promise<void> {
  const tenantId: string = 'tenant_84f3b2';
  const resetResultNoSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);

  const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const resetResultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);

  console.log(resetResultNoSso, resetResultWithSso);
}

example();
[inline-code-end]

---