Habilita ou desabilita notificações para uma página. Quando os usuários estão inscritos em uma página, notificações são criadas
para novos comentários raiz, e também

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

Retorna: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "8a3f2b1c-4d6e-4f9b-9c2d-0a1b2c3d4e5f";
const urlId: string = "article-2026-reliable-api";
const url: string = "https://blog.companyexample.com/articles/reliable-api-patterns";
const pageTitle: string = "Reliable API Patterns for Integrations";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";
const result: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
  tenantId,
  urlId,
  url,
  pageTitle,
  UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed,
  sso
);
[inline-code-end]