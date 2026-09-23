## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Não |  |
| pageSize | number | Não |  |
| afterId | string | Não |  |
| includeContext | boolean | Não |  |
| afterCreatedAt | number | Não |  |
| unreadOnly | boolean | Não |  |
| dmOnly | boolean | Não |  |
| noDm | boolean | Não |  |
| includeTranslations | boolean | Não |  |
| includeTenantNotifications | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "acme-corp";
  const urlId: string = "article-42";
  const pageSize: number = 20;
  const includeContext: boolean = true;
  const unreadOnly: boolean = true;

  const notifications: GetMyNotificationsResponse = await getUserNotifications(
    tenantId,
    urlId,
    pageSize,
    undefined,
    includeContext,
    undefined,
    unreadOnly
  );

  console.log(notifications);
}
[inline-code-end]