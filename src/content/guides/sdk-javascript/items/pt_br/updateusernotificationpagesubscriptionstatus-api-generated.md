Ativa ou desativa notificações para uma página. Quando usuários estão inscritos em uma página, notificações são criadas para novos comentários raiz, e também

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| url | string | Sim |  |
| pageTitle | string | Sim |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationStatus200Response.ts)