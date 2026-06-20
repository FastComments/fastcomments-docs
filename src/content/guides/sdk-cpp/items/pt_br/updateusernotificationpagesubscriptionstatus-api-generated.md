---
Ativar ou desativar notificações para uma página. Quando os usuários estão inscritos em uma página, notificações são criadas
para novos comentários raiz, e também

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| url | string | Sim |  |
| pageTitle | string | Sim |  |
| subscribedOrUnsubscribed | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("sso-token-abc123"));
api->updateUserNotificationPageSubscriptionStatus(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("article-456"),
    utility::conversions::to_string_t("https://www.example.com/posts/456"),
    utility::conversions::to_string_t("How to Test C++ SDK"),
    utility::conversions::to_string_t("subscribed"),
    sso
).then([](pplx::task<std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<UpdateUserNotificationPageSubscriptionStatusResponse>(*resp);
        (void)copy;
    } catch (const std::exception&) { }
});
[inline-code-end]

---