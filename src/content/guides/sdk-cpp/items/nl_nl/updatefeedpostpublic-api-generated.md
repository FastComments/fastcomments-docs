## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| updateFeedPostParams | UpdateFeedPostParams | Ja |  |
| broadcastId | string | Nee |  |
| sso | string | Nee |  |

## Antwoord

Geeft terug: [`CreateFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostPublic_200_response.h)

## Voorbeeld

[inline-code-attrs-start title = 'updateFeedPostPublic Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U"my-tenant-123";
utility::string_t postId = U"post-456";
UpdateFeedPostParams params;
boost::optional<utility::string_t> broadcastId = utility::string_t(U"broadcast-789");
boost::optional<utility::string_t> sso = utility::string_t(U"user@example.com");
api->updateFeedPostPublic(tenantId, postId, params, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<CreateFeedPostPublic_200_response>();
        (void)resp;
    } catch (...) {}
});
[inline-code-end]