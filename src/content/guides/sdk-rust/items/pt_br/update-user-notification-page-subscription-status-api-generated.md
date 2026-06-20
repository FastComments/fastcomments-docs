Habilita ou desabilita notificações para uma página. Quando usuários estão inscritos em uma página, notificações são criadas
para novos comentários raiz, e também

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| url | String | Sim |  |
| page_title | String | Sim |  |
| subscribed_or_unsubscribed | String | Sim |  |
| sso | String | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## Exemplo

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status Exemplo'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationPageSubscriptionStatusResponse, Error> {
    let params: UpdateUserNotificationPageSubscriptionStatusParams = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/rocket-launch-2026".to_string(),
        url: "https://acme.example.com/news/rocket-launch-2026".to_string(),
        page_title: "Acme Rocket Launch — June 2026".to_string(),
        subscribed_or_unsubscribed: "subscribed".to_string(),
        sso: Some("user:alice@acme.com".to_string()),
    };
    let response: UpdateUserNotificationPageSubscriptionStatusResponse =
        update_user_notification_page_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---