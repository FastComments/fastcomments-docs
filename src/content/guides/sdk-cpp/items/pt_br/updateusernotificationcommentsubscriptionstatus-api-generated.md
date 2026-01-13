---
Habilitar ou desabilitar notificações para um comentário específico.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| notificationId | string | Sim |  |
| optedInOrOut | string | Sim |  |
| commentId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateUserNotificationCommentSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
utility::string_t notificationId = utility::string_t(U("notif-789"));
utility::string_t optedInOrOut = utility::string_t(U("opted_in"));
utility::string_t commentId = utility::string_t(U("cmt-456"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::string_t(U("user@example.com")));
api->updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> t){
    try {
        auto resp = t.get();
        auto result = resp ? resp : std::make_shared<UpdateUserNotificationStatus_200_response>();
    } catch (const std::exception&) {}
});
[inline-code-end]

---