## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoReset() {
  const tenantId: string = "tenant_12345";
  const ssoToken: string = "sso_abcdef123456";

  const resultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);
  const resultWithoutSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);
}

demoReset();
[inline-code-end]

---