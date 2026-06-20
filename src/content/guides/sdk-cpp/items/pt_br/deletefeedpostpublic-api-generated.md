---
## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| postId | string | Sim |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Response

Retorna: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("feed-post-456");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->deleteFeedPostPublic(tenantId, postId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<DeleteFeedPostPublicResponse>> task){
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<DeleteFeedPostPublicResponse>();
    } catch (...) {
    }
}).wait();
[inline-code-end]

---