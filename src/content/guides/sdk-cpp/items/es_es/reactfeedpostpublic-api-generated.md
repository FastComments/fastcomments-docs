## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| postId | string | Sí |  |
| reactBodyParams | ReactBodyParams | Sí |  |
| isUndo | bool | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de reactFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("feed-post-456");
ReactBodyParams reactBodyParams;
reactBodyParams.reaction = U("like");
boost::optional<bool> isUndo = false;
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-01"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso)
.then([](std::shared_ptr<ReactFeedPostResponse> resp) {
    if (resp) {
        auto resultCopy = std::make_shared<ReactFeedPostResponse>(*resp);
    }
});
[inline-code-end]