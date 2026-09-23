Enable ou desabilite notificações para uma página. Quando os usuários estão inscritos em uma página, notificações são criadas para novos comentários raiz, e também

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
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
async function runExample() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const url: string = "https://example.com/articles/awesome-article";
  const pageTitle: string = "Awesome Article";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso_token_abc";

  const response: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  console.log(response);
}

runExample();
[inline-code-end]

---