Ativar ou desativar notificações para uma página. Quando os usuários estão inscritos em uma página, notificações são criadas
para novos comentários raiz, e também

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| url | string | Sim |  |
| pageTitle | string | Sim |  |
| subscribedOrUnsubscribed | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://example.com/articles/awesome-article");
utility::string_t pageTitle = U("Awesome Article");
utility::string_t subscription = U("subscribed");
boost::optional<utility::string_t> sso = boost::make_optional<utility::string_t>(U("sso-token-789"));

api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscription, sso)
    .then([](std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse> resp) {
        // process resp if needed
    });
[inline-code-end]