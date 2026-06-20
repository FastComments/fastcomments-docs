Comentadores anteriores na página que NÃO estão atualmente online. Ordenado por displayName.
Use isso após esgotar /users/online para renderizar uma seção "Membros".
Paginação por cursor em commenterName: o servidor percorre o índice parcial {tenantId, urlId, commenterName}
a partir de afterName para frente via $gt, sem custo de $skip.

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| afterName | string | Não |  |
| afterUserId | string | Não |  |

## Resposta

Retorna: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> afterName = boost::optional<utility::string_t>(U("jane.doe@example.com"));
boost::optional<utility::string_t> afterUserId = boost::optional<utility::string_t>(U("user-789"));
api->getOfflineUsers(tenantId, urlId, afterName, afterUserId).then([](std::shared_ptr<PageUsersOfflineResponse> resp){
    auto result = resp ? resp : std::make_shared<PageUsersOfflineResponse>();
    (void)result;
});
[inline-code-end]

---