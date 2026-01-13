## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Da |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`DeleteFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublic_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer deleteFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->deleteFeedPostPublic(tenantId, postId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<DeleteFeedPostPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<DeleteFeedPostPublic_200_response>();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]