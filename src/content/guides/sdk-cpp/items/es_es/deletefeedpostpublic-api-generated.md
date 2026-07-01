## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| options | const DeleteFeedPostPublicOptions& | Yes |  |

## Respuesta

Devuelve: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublicResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<DeleteFeedPostPublicOptions>();
opts->reason = boost::optional<utility::string_t>(U("Inappropriate content"));
api->deleteFeedPostPublic(U("my-tenant-123"), U("post-789"), *opts)
    .then([](std::shared_ptr<DeleteFeedPostPublicResponse> resp) {
        if (resp && resp->isSuccess()) {
            // lógica de éxito
        }
    });
[inline-code-end]