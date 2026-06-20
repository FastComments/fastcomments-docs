---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## Przykład

[inline-code-attrs-start title = 'Przykład updateFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
UpdateFeedPostParams updateFeedPostParams;
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->updateFeedPostPublic(tenantId, postId, updateFeedPostParams, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostResponse>> t) {
    try {
        auto resp = t.get();
        if (resp) {
            auto updatedCopy = std::make_shared<CreateFeedPostResponse>(*resp);
        }
    } catch (const std::exception& e) {
    }
});
[inline-code-end]

---